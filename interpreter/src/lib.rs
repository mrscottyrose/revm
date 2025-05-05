use crate::interpreter::{Interpreter, InterpreterResult};
use crate::primitives::{db::Database, EVMError, EVMResult, Env, Spec};
use crate::{alloc::boxed::Box, Host, InterpreterTypes};

/// Context of instruction execution. Contains Interpreter and Host.
#[derive(Debug)]
pub struct InstructionContext<'a, ITy: InterpreterTypes, H: Host + ?Sized> {
    /// Reference to Interpreter.
    pub interp: &'a mut Interpreter<ITy>,
    /// Reference to Host.
    pub host: &'a mut H,
} 