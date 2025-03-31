use anyhow::Result;

use crate::models::{command::{Command, CommandHandler}, vm::VM};

pub const COMMANDS: [Option<Command>; 3] = [
    None,
    Some(Command{mnemonics: &["ADD"], handler: &AddHandler{}}),
    Some(Command{mnemonics: &["SUB"], handler: &SubHandler{}}),
];

pub struct AddHandler;
impl CommandHandler for AddHandler {
    fn handle(&self, vm: &mut VM) -> Result<()> {
        let a = vm.pop()?;
        let b = vm.pop()?;
        vm.push(a+b)
    }
}

pub struct SubHandler;
impl CommandHandler for SubHandler {
    fn handle(&self, vm: &mut VM) -> Result<()> {
        todo!()
    }
}

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
        vm.push(2).unwrap();
        vm.push(3).unwrap();

        AddHandler{}.handle(&mut vm).unwrap();

        assert_eq!(vm.read_stack(0).unwrap(), 5)
    }
}
