pub struct Command<'a> {
    pub code: i32,
    pub mnemonic: &'a str,
    pub handler: CommandHandler,
}

pub type CommandHandler = fn() -> ();
