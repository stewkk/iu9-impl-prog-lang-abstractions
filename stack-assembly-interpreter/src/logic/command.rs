use anyhow::{anyhow, Result};

use crate::models::{command::{Command, CommandHandler, Opcode}, vm::VM};

pub const COMMANDS: [Option<Command>; 37] = [
    Some(Command{mnemonics: &["ADD"], handler: &AddHandler{}}),
    Some(Command{mnemonics: &["SUB"], handler: &SubHandler{}}),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Command{mnemonics: &["HALT"], handler: &HaltHandler{}}),
];

pub fn get_handler(opcode: Opcode) -> Result<&'static dyn CommandHandler> {
    let index = opcode as usize - 1;
    COMMANDS.get(index)
            .map(Option::as_ref)
            .flatten()
            .ok_or_else(|| anyhow!("no handler for opcode {opcode}"))
            .map(|x| x.handler)
}

pub struct AddHandler;
impl CommandHandler for AddHandler {
    fn handle(&self, vm: &mut VM) -> Result<()> {
        let y = vm.pop()?;
        let x = vm.pop()?;
        vm.push(x+y)
    }
}

pub struct SubHandler;
impl CommandHandler for SubHandler {
    fn handle(&self, vm: &mut VM) -> Result<()> {
        let y = vm.pop()?;
        let x = vm.pop()?;
        vm.push(x-y)
    }
}

pub struct HaltHandler;
impl CommandHandler for HaltHandler {
    fn handle(&self, vm: &mut VM) -> Result<()> {
        Ok(())
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
