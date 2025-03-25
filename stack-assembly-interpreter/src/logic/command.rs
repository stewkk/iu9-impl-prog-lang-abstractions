use crate::models::command::Command;

pub static COMMANDS: &[Command] = &[
    Command{code: -1, mnemonic: "ADD", handler: AddHandler},
    Command{code: -2, mnemonic: "SUB", handler: SubHandler},
];

fn AddHandler() -> () {}

fn SubHandler() -> () {}
