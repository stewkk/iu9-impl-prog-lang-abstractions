pub type Opcode = i32;

pub struct Command<'a> {
    pub mnemonics: &'a [&'a str],
    pub handler: CommandHandler,
}

pub type CommandHandler = fn() -> ();
