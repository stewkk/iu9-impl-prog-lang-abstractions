use crate::models::command::Command;

pub static COMMANDS: &[Command] = &[
    Command{code: -1, mnemonic: "ADD", handler: add_handler},
    Command{code: -2, mnemonic: "SUB", handler: sub_handler},
];

fn add_handler() -> () {}

fn sub_handler() -> () {}
