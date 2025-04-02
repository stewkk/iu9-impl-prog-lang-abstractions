use anyhow::Result;

use super::token::Token;
use super::vm::VM;

pub type Opcode = i64;

#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub opcode: Opcode,
    pub token: Token,
}

pub struct Command<'a> {
    pub mnemonics: &'a [&'a str],
    pub handler: &'static dyn CommandHandler,
}

pub type ReturnCode = i64;

pub trait CommandHandler {
    fn handle(&self, vm: &mut VM, io: &dyn InputOutput) -> Result<Option<ReturnCode>>;
}

pub trait Input {
    fn get_char(&self) -> Result<i64>;
}

pub trait Output {
    fn print_char(&self, c: i64) -> Result<()>;
}

pub trait InputOutput: Input + Output {}
impl<T: Input + Output> InputOutput for T {}
