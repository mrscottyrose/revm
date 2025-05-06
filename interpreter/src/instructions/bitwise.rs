use crate::{
    gas,
    primitives::{Spec::*, U256},
    Host,
    InstructionContext, InterpreterResult,
    interpreter_types::InterpreterTypes,
};

/// Bitwise less than comparison.
#[inline]
pub fn lt<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    gas!(context.interp, gas::VERYLOW);
    context.interp.stack.eval_top(|op1, op2| (U256::from(op1 < op2), false))?;
    Ok(())
}

/// Bitwise greater than comparison.
#[inline]
pub fn gt<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    gas!(context.interp, gas::VERYLOW);
    context.interp.stack.eval_top(|op1, op2| (U256::from(op1 > op2), false))?;
    Ok(())
}

/// Signed less than comparison.
#[inline]
pub fn slt<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    gas!(context.interp, gas::VERYLOW);
    context
        .interp
        .stack
        .eval_top(|op1, op2| (U256::from(op1.slt(op2)), false))?;
    Ok(())
}

/// Signed greater than comparison.
#[inline]
pub fn sgt<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    gas!(context.interp, gas::VERYLOW);
    context
        .interp
        .stack
        .eval_top(|op1, op2| (U256::from(op1.sgt(op2)), false))?;
    Ok(())
}

/// Bitwise equals comparison.
#[inline]
pub fn eq<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    gas!(context.interp, gas::VERYLOW);
    context
        .interp
        .stack
        .eval_top(|op1, op2| (U256::from(op1 == op2), false))?;
    Ok(())
}

/// Checks if a stack item is zero.
#[inline]
pub fn iszero<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    gas!(context.interp, gas::VERYLOW);
    // pop exactly one value, test for zero, then push the result
    let value = context.interp.stack.pop_u256()?;
    context.interp.stack.push(U256::from(value == U256::ZERO))?;
    Ok(())
}

/// Bitwise AND operation.
#[inline]
pub fn bitand<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    gas!(context.interp, gas::VERYLOW);
    context.interp.stack.eval_top(|op1, op2| (op1 & op2, false))?;
    Ok(())
}

/// Bitwise OR operation.
#[inline]
pub fn bitor<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    gas!(context.interp, gas::VERYLOW);
    context.interp.stack.eval_top(|op1, op2| (op1 | op2, false))?;
    Ok(())
}

/// Bitwise XOR operation.
#[inline]
pub fn bitxor<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    gas!(context.interp, gas::VERYLOW);
    context.interp.stack.eval_top(|op1, op2| (op1 ^ op2, false))?;
    Ok(())
}

/// Bitwise NOT operation.
#[inline]
pub fn not<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    gas!(context.interp, gas::VERYLOW);
    let value = context.interp.stack.pop_u256()?;
    context.interp.stack.push(!value)?;
    Ok(())
}

/// Retrieves a single byte from a stack item.
#[inline]
pub fn byte<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    gas!(context.interp, gas::VERYLOW);
    context.interp.stack.eval_top(|op1, op2| (op2.byte(op1), false))?;
    Ok(())
}

/// Left shift operation.
#[inline]
pub fn shl<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    // EIP-145: Bitwise shifting instructions
    check!(context.interp, Constantinople);
    gas!(context.interp, gas::VERYLOW);
    context.interp.stack.eval_top(|op1, op2| (op2 << op1, false))?;
    Ok(())
}

/// Logical right shift operation.
#[inline]
pub fn shr<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    // EIP-145: Bitwise shifting instructions
    check!(context.interp, Constantinople);
    gas!(context.interp, gas::VERYLOW);
    context.interp.stack.eval_top(|op1, op2| (op2 >> op1, false))?;
    Ok(())
}

/// Arithmetic right shift operation.
#[inline]
pub fn sar<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    // EIP-145: Bitwise shifting instructions
    check!(context.interp, Constantinople);
    gas!(context.interp, gas::VERYLOW);
    context.interp.stack.eval_top(|op1, op2| (op2.sar(op1), false))?;
    Ok(())
} 