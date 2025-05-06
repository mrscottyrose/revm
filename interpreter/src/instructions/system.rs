use crate::{
    gas,
    Host,
    InstructionContext, InterpreterResult, InstructionResult, InterpreterTypes,
};

/// Halts execution and returns the data from the specified memory range.
#[inline]
pub fn return_op<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    pop_memory_range!(context.interp, offset, len);
    gas!(context.interp, gas::explicit_memory_cost(len));
    context.interp.shared_memory.resize_offset(offset, len)?;
    context.interp.return_data_buffer =
        context.interp.shared_memory.slice_range(offset, len);
    Err(InstructionResult::Return)
}

/// Reverts execution and returns the data from the specified memory range.
#[inline]
pub fn revert<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    // EIP-140: REVERT instruction
    check!(context.interp, Byzantium);
    pop_memory_range!(context.interp, offset, len);
    gas!(context.interp, gas::explicit_memory_cost(len));
    context.interp.shared_memory.resize_offset(offset, len)?;
    context.interp.return_data_buffer =
        context.interp.shared_memory.slice_range(offset, len);
    Err(InstructionResult::Revert)
}

/// Halts execution.
#[inline]
pub fn stop<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    Err(InstructionResult::Stop)
}

/// Halts execution and marks it as invalid.
#[inline]
pub fn invalid<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    Err(InstructionResult::InvalidFEOpcode)
}

/// Halts execution and marks the contract as self-destructed.
#[inline]
pub fn selfdestruct<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    check_staticcall!(context.interp);
    check!(context.interp, Berlin);

    pop_address!(context.interp, target);

    let res = context.host.selfdestruct(context.interp.contract.address, target)?;

    // Base SELFDESTRUCT cost including cold-account surcharge.
    gas!(context.interp, gas::selfdestruct_cost(context.interp.spec(), res));
    gas!(context.interp, gas::selfdestruct_gas_topup(
        context.interp.spec(),
        res,
        target
    ));
    Err(InstructionResult::SelfDestruct)
}

/// Halts execution and marks the contract as self-destructed.
#[inline]
pub fn selfdestruct_pre_berlin<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    check_staticcall!(context.interp);
    gas!(context.interp, gas::SELFDESTRUCT);

    pop_address!(context.interp, target);

    let res = context.host.selfdestruct(context.interp.contract.address, target)?;
    gas!(context.interp, gas::selfdestruct_gas_topup(
        context.interp.spec(),
        res,
        target
    ));
    debug_assert!(!res.previously_destroyed, "Impossible case");
    Err(InstructionResult::SelfDestruct)
} 