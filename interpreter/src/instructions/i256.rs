use crate::{
    gas,
    primitives::{Spec, I256, U256},
    Host,
    InstructionContext, InterpreterResult,
    interpreter_types::InterpreterTypes,
};

/// Signed division operation.
#[inline]
pub fn sdiv<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    gas!(context.interp, gas::LOW);
    context.interp.stack.eval_top(|op1, op2| {
        let op1: I256 = op1.into();
        let op2: I256 = op2.into();
        (op1.div(op2).into(), false)
    })?;
    Ok(())
}

/// Signed modulo operation.
#[inline]
pub fn smod<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    gas!(context.interp, gas::LOW);
    context.interp.stack.eval_top(|op1, op2| {
        let op1: I256 = op1.into();
        let op2: I256 = op2.into();
        (op1.rem(op2).into(), false)
    })?;
    Ok(())
} 