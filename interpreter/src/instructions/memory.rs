use crate::{
    gas, interpreter::Interpreter,
    primitives::{Spec, U256},
    Host,
    InstructionContext, InterpreterResult,
};

/// Load a word from memory.
#[inline]
pub fn mload<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    gas!(context.interp, gas::VERYLOW);
    let offset_u256 = context.interp.stack.pop()?;
    let offset: usize = offset_u256
        .try_into()
        .map_err(|_| crate::InstructionResult::InvalidMemoryOffset)?;
    context.interp.shared_memory.resize_offset(offset, 32)?;
    let value = context.interp.shared_memory.get_word(offset);
    context.interp.stack.push(value)?;
    Ok(())
}

/// Store a word in memory.
#[inline]
pub fn mstore<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    gas!(context.interp, gas::VERYLOW);
    let value = context.interp.stack.pop()?;
    let offset_u256 = context.interp.stack.pop()?;
    let offset: usize = offset_u256
        .try_into()
        .map_err(|_| crate::InstructionResult::InvalidMemoryOffset)?;
    context.interp.shared_memory.resize_offset(offset, 32)?;
    context.interp.shared_memory.set_word(offset, value);
    Ok(())
}

/// Store a byte in memory.
#[inline]
pub fn mstore8<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    gas!(context.interp, gas::VERYLOW);
    let value = context.interp.stack.pop()?;
    let offset_u256 = context.interp.stack.pop()?;
    let offset: usize = offset_u256
        .try_into()
        .map_err(|_| crate::InstructionResult::InvalidMemoryOffset)?;
    context.interp.shared_memory.resize_offset(offset, 1)?;
    let value = value.byte(31);
    // SAFETY: we resized the memory two lines above.
    unsafe { context.interp.shared_memory.set_byte(offset, value) };
    Ok(())
}

/// Pushes the size of the active memory onto the stack.
#[inline]
pub fn msize<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    gas!(context.interp, gas::BASE);
    context.interp.stack.push(U256::from(context.interp.shared_memory.len()))?;
    Ok(())
} 