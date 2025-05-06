use crate::{
    gas,
    primitives::{Spec, U256},
    interpreter_types::InterpreterTypes,
    Host,
    InstructionContext, InterpreterResult,
};

/// EIP-1: JUMP instruction
#[inline]
pub fn jump<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    gas!(context.interp, gas::MID);
    let dest = context.interp.stack.pop_u256()?;
    context.interp.instruction_pointer = context.interp.validate_jump_dest(dest)?;
    Ok(())
}

/// EIP-1: JUMPI instruction
#[inline]
pub fn jumpi<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    gas!(context.interp, gas::HIGH);
    let dest = context.interp.stack.pop_u256()?;
    let value = context.interp.stack.pop()?;

    if value != U256::ZERO {
        context.interp.instruction_pointer = context.interp.validate_jump_dest(dest)?;
    }
    Ok(())
}

/// EIP-615: JUMPDEST instruction
#[inline]
pub fn jumpdest<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    gas!(context.interp, gas::JUMPDEST);
    Ok(())
}

/// EIP-1153: Transient storage opcode. TSTORE
#[inline]
pub fn tstore<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    // Opcode is only enabled in Cancun.
    check!(context.interp, Cancun);
    // Disallow writes during STATICCALL.
    check_staticcall!(context.interp);
    gas!(context.interp, gas::TSTORE);
    // EIP-1153 expects (value, key) at the top of the stack.
    let value = context.interp.stack.pop()?;
    let key   = context.interp.stack.pop()?;
    context.host.tstore(key, value)
}

/// EIP-1153: Transient storage opcode. TLOAD
#[inline]
pub fn tload<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    // Opcode is only enabled in Cancun.
    check!(context.interp, Cancun);
    gas!(context.interp, gas::TLOAD);
    let key = context.interp.stack.pop()?;
    let value = context.host.tload(key)?;
    context.interp.stack.push(value)?;
    Ok(())
}

/// Pushes the program counter onto the stack.
#[inline]
pub fn pc<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    gas!(context.interp, gas::BASE);
    context
        .interp
        .stack
        .push(U256::from(context.interp.instruction_pointer as u64))?;
    Ok(())
}

/// Pushes the gas left onto the stack.
#[inline]
pub fn gas<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    gas!(context.interp, gas::BASE);
    context.interp.stack.push(U256::from(context.interp.gas.remaining()))?;
    Ok(())
} 