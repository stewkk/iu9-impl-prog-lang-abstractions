use anyhow::{anyhow, Result};

use crate::models::{command::{Command, CommandHandler, InputOutput, Opcode, ReturnCode}, vm::VM};

macro_rules! bin_op_handler {
    ( $handler:ident, $op:tt ) => {
        pub struct $handler;
        impl CommandHandler for $handler {
            fn handle(&self, vm: &mut VM, _: &dyn InputOutput) -> Result<Option<ReturnCode>> {
                let y = vm.pop()?;
                let x = vm.pop()?;
                vm.push(x $op y)?;
                Ok(None)
            }
        }
    };
}

macro_rules! get_register_handler {
    ( $handler:ident, $register:ident ) => {
        pub struct $handler;
        impl CommandHandler for $handler {
            fn handle(&self, vm: &mut VM, _: &dyn InputOutput) -> Result<Option<ReturnCode>> {
                vm.push(vm.registers().$register)?;
                Ok(None)
            }
        }
    };
}

macro_rules! set_register_handler {
    ( $handler:ident, $register:ident ) => {
        pub struct $handler;
        impl CommandHandler for $handler {
            fn handle(&self, vm: &mut VM, _: &dyn InputOutput) -> Result<Option<ReturnCode>> {
                let a = vm.pop()?;
                vm.registers_mut().$register = a;
                Ok(None)
            }
        }
    };
}

pub const COMMANDS: [Option<Command>; 44] = [
    Some(Command{mnemonics: &["ADD"], handler: &AddHandler{}}),
    Some(Command{mnemonics: &["SUB"], handler: &SubHandler{}}),
    Some(Command{mnemonics: &["BITAND"], handler: &BitwiseAndHandler{}}),
    Some(Command{mnemonics: &["BITOR"], handler: &BitwiseOrHandler{}}),
    Some(Command{mnemonics: &["BITXOR"], handler: &BitwiseXorHandler{}}),
    Some(Command{mnemonics: &["LSHIFT"], handler: &LeftShiftHandler{}}),
    Some(Command{mnemonics: &["RSHIFT"], handler: &RightShiftHandler{}}),
    Some(Command{mnemonics: &["CMP"], handler: &CmpHandler{}}),
    Some(Command{mnemonics: &["GETIP"], handler: &GetIPHandler{}}),
    Some(Command{mnemonics: &["GETSP"], handler: &GetSPHandler{}}),
    Some(Command{mnemonics: &["GETFP"], handler: &GetFPHandler{}}),
    Some(Command{mnemonics: &["GETRV"], handler: &GetRVHandler{}}),
    Some(Command{mnemonics: &["SETIP"], handler: &SetIPHandler{}}),
    Some(Command{mnemonics: &["SETSP"], handler: &SetSPHandler{}}),
    Some(Command{mnemonics: &["SETFP"], handler: &SetFPHandler{}}),
    Some(Command{mnemonics: &["SETRV"], handler: &SetRVHandler{}}),
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
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Command{mnemonics: &["OUT"], handler: &OutHandler{}}),
];

pub fn get_handler(opcode: Opcode) -> Result<&'static dyn CommandHandler> {
    let index = -opcode as usize - 1;
    COMMANDS.get(index)
            .map(Option::as_ref)
            .flatten()
            .ok_or_else(|| anyhow!("no handler for opcode {opcode}"))
            .map(|x| x.handler)
}

bin_op_handler!(AddHandler, +);
bin_op_handler!(SubHandler, -);
bin_op_handler!(BitwiseAndHandler, &);
bin_op_handler!(BitwiseOrHandler, |);
bin_op_handler!(BitwiseXorHandler, ^);
bin_op_handler!(LeftShiftHandler, <<);
bin_op_handler!(RightShiftHandler, >>);

get_register_handler!(GetIPHandler, ip);
get_register_handler!(GetFPHandler, fp);
get_register_handler!(GetSPHandler, sp);
get_register_handler!(GetRVHandler, rv);

set_register_handler!(SetIPHandler, ip);
set_register_handler!(SetFPHandler, fp);
set_register_handler!(SetSPHandler, sp);
set_register_handler!(SetRVHandler, rv);

pub struct HaltHandler;
impl CommandHandler for HaltHandler {
    fn handle(&self, vm: &mut VM, _: &dyn InputOutput) -> Result<Option<ReturnCode>> {
        let rc = vm.pop()?;
        Ok(Some(rc))
    }
}

pub struct OutHandler;
impl CommandHandler for OutHandler {
    fn handle(&self, vm: &mut VM, io: &dyn InputOutput) -> Result<Option<ReturnCode>> {
        let a = vm.pop()?;
        io.print_char(a)?;
        Ok(None)
    }
}

pub struct CmpHandler;
impl CommandHandler for CmpHandler {
    fn handle(&self, vm: &mut VM, _: &dyn InputOutput) -> Result<Option<ReturnCode>> {
        let y = vm.pop()?;
        let x = vm.pop()?;
        if x < y {
            vm.push(-1)?;
        } else if x > y {
            vm.push(1)?;
        } else {
            vm.push(0)?;
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use crate::{logic::stdio::Stdio, models::{command::Instruction, token::{Position, Token}, vm::VM}};

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

        AddHandler{}.handle(&mut vm, &Stdio{}).unwrap();

        assert_eq!(vm.read_stack(0).unwrap(), 5)
    }
}
