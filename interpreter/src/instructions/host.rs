use crate::{
    gas,
    primitives::{Spec::*, Address, B256, U256},
    Host,
    InstructionContext, InterpreterResult,
    interpreter_types::InterpreterTypes,
};

/// SLOAD instruction (EIP-2929/BERLIN variant)
#[inline]
pub fn sload<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    check!(context.interp, Istanbul);
    // Warm-access cost first; the cold surcharge (if any) is added later.
    gas!(context.interp, gas::sload_cost(context.interp.spec(), /*is_cold=*/false));

    let index = context.interp.stack.pop()?;
    let (value, is_cold) = context.host.sload(context.interp.contract.address, index)?;
    gas!(context.interp, gas::cold_sload_cost(context.interp.spec(), is_cold));
    context.interp.stack.push(value)?;
    Ok(())
}

/// EIP-1153: Transient storage opcode. TSTORE
#[inline]
pub fn sload_pre_istanbul<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    gas!(context.interp, gas::SLOAD);

    let index = context.interp.stack.pop()?;
    let (value, is_cold) = context.host.sload(context.interp.contract.address, index)?;
    debug_assert!(!is_cold);
    context.interp.stack.push(value)?;
    Ok(())
}

/// EIP-1153: Transient storage opcode. TSTORE
#[inline]
pub fn sstore<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    check_staticcall!(context.interp);
    check!(context.interp, Istanbul);

    let index = context.interp.stack.pop()?;
    let value = context.interp.stack.pop()?;

    let (original, old, new, is_cold) =
        context
            .host
            .sstore(context.interp.contract.address, index, value)?;
    let sstore_result = gas::SStoreResult { original, old, new };
    let gas_cost = gas::sstore_cost(context.interp.spec(), &sstore_result, is_cold);
    gas_or_fail!(context.interp, gas_cost);
    context.interp.gas.record_refund(context.host.sstore_refund());
    Ok(())
}

/// EIP-1153: Transient storage opcode. TSTORE
#[inline]
pub fn sstore_pre_istanbul<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    check_staticcall!(context.interp);

    let index = context.interp.stack.pop()?;
    let value = context.interp.stack.pop()?;

    // Note: it is possible to provide more information to host then just address/index/value
    let (original, old, new, is_cold) =
        context
            .host
            .sstore(context.interp.contract.address, index, value)?;
    let gas_cost = gas::sstore_cost_pre_istanbul(original, old, new);
    gas_or_fail!(context.interp, gas_cost);
    context.interp.gas.record_refund(context.host.sstore_refund());
    debug_assert!(!is_cold);
    Ok(())
}

/// Send LOG0 opcode.
#[inline]
pub fn log0<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    common_log(context, 0)
}

/// Send LOG1 opcode.
#[inline]
pub fn log1<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    common_log(context, 1)
}

/// Send LOG2 opcode.
#[inline]
pub fn log2<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    common_log(context, 2)
}

/// Send LOG3 opcode.
#[inline]
pub fn log3<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    common_log(context, 3)
}

/// Send LOG4 opcode.
#[inline]
pub fn log4<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    common_log(context, 4)
}

#[inline]
fn common_log<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
    n: usize,
) -> InterpreterResult {
    check_staticcall!(context.interp);
    pop_memory_range!(context.interp, offset, len);
    // Calculate gas cost.
    gas_or_fail!(context.interp, gas::log_cost(n as u8, len as u64));
    let data = context.interp.shared_memory.slice_range(offset, len);

    let mut topics = crate::alloc::vec::Vec::with_capacity(n);
    for _ in 0..n {
        topics.push(B256(context.interp.stack.pop()?.to_be_bytes()));
    }

    context.host.log(context.interp.contract.address, topics, data.into())
}

/// Create a new contract.
#[inline]
pub fn create<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    common_create(context, false)
}

/// Create a new contract with CREATE2.
#[inline]
pub fn create2<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    common_create(context, true)
}

/// Common logic for CREATE and CREATE2
#[inline]
fn common_create<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
    is_create2: bool,
) -> InterpreterResult {
    check_staticcall!(context.interp);

    let value = context.interp.stack.pop()?;
    pop_memory_range!(context.interp, code_offset, len);

    let code = context.interp.shared_memory.slice_range(code_offset, len);

    let salt = if is_create2 {
        check!(context.interp, Constantinople);
        let salt = context.interp.stack.pop()?;
        gas_or_fail!(context.interp, gas::create2_cost(len));
        Some(salt)
    } else {
        gas!(context.interp, gas::CREATE);
        None
    };

    let gas = context.interp.gas();
    // EIP-150: Gas cost changes for IO-heavy operations
    if context.interp.call_depth >= 1024 {
        return Err(InstructionResult::CallDepthOverflow.into());
    }
    let remaining_gas = gas.remaining();
    // Max gas for call is 63/64 of remaining gas.
    let mut create_gas = remaining_gas.saturating_sub(remaining_gas / 64);

    // Reduce gas cost of Shanghai.
    if context.interp.spec().enabled(Shanghai) {
        // TODO gas initcode cost
    }

    // Reserve the sub-call gas by taking it rather than charging it as cost.
    // Immediately return OutOfGas if the take fails.
    gas.take(create_gas)
        .map_err(|_| InstructionResult::OutOfGas)?; // Assuming take returns Result<(), Error>

    let mut call_result = context.host.create(
        context.interp.contract.caller,
        value,
        code.into(),
        salt,
        create_gas, // This is the gas limit for the sub-call
    )?;

    // Reconcile gas after the sub-call returns.
    gas.refund_unused(call_result.gas_left);
    gas.record_refund(call_result.gas_refund); // Keep existing explicit gas refund mechanism

    let created_address = if call_result.result.is_error() {
        U256::ZERO
    } else {
        U256::from_be_bytes(call_result.address.map(|a| a.0).unwrap_or_default())
    };

    context.interp.stack.push(created_address)?;

    Ok(())
} 