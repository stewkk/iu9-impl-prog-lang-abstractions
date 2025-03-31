use super::token::Token;
use super::vm::VM;

pub type Opcode = i32;

#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub opcode: Opcode,
    pub token: Token,
}

pub struct Command<'a> {
    pub mnemonics: &'a [&'a str],
    pub handler: CommandHandler,
}

pub trait CommandHandler {
    fn handle(vm: &mut VM) -> () {

    }
}

// pub type CommandHandler = fn(vm: &mut VM) -> ();
