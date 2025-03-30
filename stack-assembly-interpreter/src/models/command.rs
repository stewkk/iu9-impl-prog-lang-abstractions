use super::token::Token;

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

pub type CommandHandler = fn() -> ();
