use crate::{
    gas,
    primitives::{Spec, U256},
    Host,
    InstructionContext, InterpreterResult,
    interpreter_types::InterpreterTypes,
};

/// Adds the top two stack items.
#[inline]
pub fn add<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    gas!(context.interp, gas::VERYLOW);
    context.interp.stack.eval_top(|op1, op2| op1.overflowing_add(op2))?;
    Ok(())
}

/// Multiplies the top two stack items.
#[inline]
pub fn mul<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    gas!(context.interp, gas::LOW);
    context.interp.stack.eval_top(|op1, op2| op1.overflowing_mul(op2))?;
    Ok(())
}

/// Subtracts the top two stack items.
#[inline]
pub fn sub<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    gas!(context.interp, gas::VERYLOW);
    context.interp.stack.eval_top(|op1, op2| op1.overflowing_sub(op2))?;
    Ok(())
}

/// Performs integer division on the top two stack items.
#[inline]
pub fn div<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    gas!(context.interp, gas::LOW);
    context.interp.stack.eval_top(|op1, op2| {
        if op2 == U256::ZERO {
            (U256::ZERO, false)
        } else {
            (op1 / op2, false)
        }
    })?;
    Ok(())
}

/// Performs modulo operation on the top two stack items.
#[inline]
pub fn rem<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    gas!(context.interp, gas::LOW);
    context.interp.stack.eval_top(|op1, op2| {
        if op2 == U256::ZERO {
            (U256::ZERO, false)
        } else {
            (op1 % op2, false)
        }
    })?;
    Ok(())
}

/// Adds the top three stack items.
#[inline]
pub fn addmod<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    gas!(context.interp, gas::MID);
    context
        .interp
        .stack
        .eval_top3(|op1, op2, op3| op1.add_mod(op2, op3))?;
    Ok(())
}

/// Multiplies the top three stack items.
#[inline]
pub fn mulmod<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    gas!(context.interp, gas::MID);
    context
        .interp
        .stack
        .eval_top3(|op1, op2, op3| op1.mul_mod(op2, op3))?;
    Ok(())
}

/// Takes the exponent of the top two stack items.
#[inline]
pub fn exp<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    gas!(context.interp, gas::exp_cost(context.interp.spec(), context.interp.stack.peek_at(1)?));
    context.interp.stack.eval_top(|op1, op2| op1.pow(op2))?;
    Ok(())
}

/// Sign extends the first stack item using the second stack item as the sign bit location.
///
/// Sign extend `op1` from `(op2 * 8 + 7)`th bit in `op1`.
#[inline]
pub fn signextend<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    gas!(context.interp, gas::LOW);
    context.interp.stack.eval_top(|op1, op2| (op1.sign_extend_from(op2), false))?;
    Ok(())
} 