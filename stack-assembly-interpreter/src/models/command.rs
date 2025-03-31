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

pub trait CommandHandler {
    fn handle(&self, vm: &mut VM) -> Result<()>;
}

