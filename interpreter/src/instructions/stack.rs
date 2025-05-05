use crate::{
    gas,
    Host,
    InstructionContext, InterpreterResult, InterpreterTypes,
};

/// Remove item from stack.
#[inline]
pub fn pop<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    gas!(context.interp, gas::BASE);
    context.interp.stack.pop()?;
    Ok(())
}

/// Duplicate stack item `N`.
#[macro_export]
macro_rules! dup {
    ($context:expr, $n:expr) => {{ 
        gas!($context.interp, gas::VERYLOW); 
        if $context.interp.stack.dup::<$n>() { 
            Ok(()) 
        } else { 
            Err(crate::InstructionResult::StackUnderflow.into()) 
        } 
    }}; 
}

/// Swap stack item `N` with top.
#[macro_export]
macro_rules! swap {
    ($context:expr, $n:expr) => {{ 
        gas!($context.interp, gas::VERYLOW); 
        if $context.interp.stack.swap::<$n>() { 
            Ok(()) 
        } else { 
            Err(crate::InstructionResult::StackUnderflow.into()) 
        } 
    }}; 
}

/// Duplicate 1st stack item.
#[inline]
pub fn dup1<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    dup!(context, 1)
}

/// Duplicate 2nd stack item.
#[inline]
pub fn dup2<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    dup!(context, 2)
}

/// Duplicate 3rd stack item.
#[inline]
pub fn dup3<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    dup!(context, 3)
}

/// Duplicate 4th stack item.
#[inline]
pub fn dup4<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    dup!(context, 4)
}

/// Duplicate 5th stack item.
#[inline]
pub fn dup5<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    dup!(context, 5)
}

/// Duplicate 6th stack item.
#[inline]
pub fn dup6<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    dup!(context, 6)
}

/// Duplicate 7th stack item.
#[inline]
pub fn dup7<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    dup!(context, 7)
}

/// Duplicate 8th stack item.
#[inline]
pub fn dup8<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    dup!(context, 8)
}

/// Duplicate 9th stack item.
#[inline]
pub fn dup9<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    dup!(context, 9)
}

/// Duplicate 10th stack item.
#[inline]
pub fn dup10<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    dup!(context, 10)
}

/// Duplicate 11th stack item.
#[inline]
pub fn dup11<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    dup!(context, 11)
}

/// Duplicate 12th stack item.
#[inline]
pub fn dup12<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    dup!(context, 12)
}

/// Duplicate 13th stack item.
#[inline]
pub fn dup13<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    dup!(context, 13)
}

/// Duplicate 14th stack item.
#[inline]
pub fn dup14<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    dup!(context, 14)
}

/// Duplicate 15th stack item.
#[inline]
pub fn dup15<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    dup!(context, 15)
}

/// Duplicate 16th stack item.
#[inline]
pub fn dup16<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    dup!(context, 16)
}

/// Swap 1st stack item with top.
#[inline]
pub fn swap1<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    swap!(context, 1)
}

/// Swap 2nd stack item with top.
#[inline]
pub fn swap2<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    swap!(context, 2)
}

/// Swap 3rd stack item with top.
#[inline]
pub fn swap3<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    swap!(context, 3)
}

/// Swap 4th stack item with top.
#[inline]
pub fn swap4<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    swap!(context, 4)
}

/// Swap 5th stack item with top.
#[inline]
pub fn swap5<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    swap!(context, 5)
}

/// Swap 6th stack item with top.
#[inline]
pub fn swap6<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    swap!(context, 6)
}

/// Swap 7th stack item with top.
#[inline]
pub fn swap7<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    swap!(context, 7)
}

/// Swap 8th stack item with top.
#[inline]
pub fn swap8<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    swap!(context, 8)
}

/// Swap 9th stack item with top.
#[inline]
pub fn swap9<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    swap!(context, 9)
}

/// Swap 10th stack item with top.
#[inline]
pub fn swap10<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    swap!(context, 10)
}

/// Swap 11th stack item with top.
#[inline]
pub fn swap11<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    swap!(context, 11)
}

/// Swap 12th stack item with top.
#[inline]
pub fn swap12<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    swap!(context, 12)
}

/// Swap 13th stack item with top.
#[inline]
pub fn swap13<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    swap!(context, 13)
}

/// Swap 14th stack item with top.
#[inline]
pub fn swap14<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    swap!(context, 14)
}

/// Swap 15th stack item with top.
#[inline]
pub fn swap15<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    swap!(context, 15)
}

/// Swap 16th stack item with top.
#[inline]
pub fn swap16<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    swap!(context, 16)
} 