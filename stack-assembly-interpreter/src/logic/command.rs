use crate::models::command::{Command, CommandHandler};

pub static COMMANDS: &[Option<Command>] = &[
    None,
    Some(Command{mnemonics: &["ADD"], handler: ADD_HANDLER}),
    Some(Command{mnemonics: &["SUB"], handler: SUB_HANDLER}),
];

static ADD_HANDLER: CommandHandler = |_vm| {
    ()
};

static SUB_HANDLER: CommandHandler = |_vm| {
    ()
};

#[cfg(test)]
mod tests {
    use crate::models::{command::Instruction, token::{Position, Token}, vm::VM};

    use super::*;

    #[test]
    fn add_handler() {
        let mut vm = VM::new(vec![
            Instruction{
                opcode: -1,
                token: Token::Ident(
                    "ADD".to_string(),
                    Position{filename: "test".to_string(), line: 1, column: 3}
                ),
            }
        ]);

        ADD_HANDLER(&mut vm);

        assert_eq!(vm.read_stack(0).unwrap(), 5)
    }
}
