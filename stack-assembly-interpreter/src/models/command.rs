pub type Opcode = i32;

pub struct Command<'a> {
    pub code: Opcode,
    pub mnemonic: &'a str,
    pub handler: CommandHandler,
}

pub type CommandHandler = fn() -> ();
