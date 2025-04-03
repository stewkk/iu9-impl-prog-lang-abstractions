use anyhow::{anyhow, Result};

use crate::models::{command::{Command, CommandHandler, InputOutput, Opcode, ReturnCode}, vm::VM};

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
    Some(Command{mnemonics: &["DROP2"], handler: &Drop2Handler{}}),
    Some(Command{mnemonics: &["DUP"], handler: &DupHandler{}}),
    Some(Command{mnemonics: &["DROP"], handler: &DropHandler{}}),
    Some(Command{mnemonics: &["SWAP"], handler: &SwapHandler{}}),
    Some(Command{mnemonics: &["ROT"], handler: &RotHandler{}}),
    Some(Command{mnemonics: &["OVER"], handler: &OverHandler{}}),
    Some(Command{mnemonics: &["SDROP"], handler: &SdropHandler{}}),
    None,
    None,
    Some(Command{mnemonics: &["NEG"], handler: &NegHandler{}}),
    Some(Command{mnemonics: &["BITNOT"], handler: &BitwiseNotHandler{}}),
    Some(Command{mnemonics: &["LOAD"], handler: &LoadHandler{}}),
    Some(Command{mnemonics: &["SAVE"], handler: &SaveHandler{}}),
    Some(Command{mnemonics: &["HALT"], handler: &HaltHandler{}}),
    None,
    None,
    Some(Command{mnemonics: &["MUL"], handler: &MulHandler{}}),
    Some(Command{mnemonics: &["DIV"], handler: &DivHandler{}}),
    Some(Command{mnemonics: &["MOD"], handler: &ModHandler{}}),
    Some(Command{mnemonics: &["IN"], handler: &InHandler{}}),
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

macro_rules! handler {
    ( $handler:ident, $body:ident ) => {
        pub struct $handler;
        impl CommandHandler for $handler {
            fn handle(&self, vm: &mut VM, _: &dyn InputOutput) -> Result<Option<ReturnCode>> {
                $body(vm)?;
                Ok(None)
            }
        }
    };
}

macro_rules! bin_op_handler {
    ( $handler:ident, $body:ident, $op:tt ) => {
        fn $body(vm: &mut VM) -> Result<()> {
            let y = vm.pop()?;
            let x = vm.pop()?;
            vm.push(x $op y)?;
            Ok(())
        }

        handler!($handler, $body);
    };
}

macro_rules! get_register_handler {
    ( $handler:ident, $body:ident, $register:ident ) => {
        fn $body(vm: &mut VM) -> Result<()> {
            vm.push(vm.registers().$register)?;
            Ok(())
        }

        handler!($handler, $body);
    };
}

macro_rules! set_register_handler {
    ( $handler:ident, $body:ident, $register:ident ) => {
        fn $body(vm: &mut VM) -> Result<()> {
            let a = vm.pop()?;
            vm.registers_mut().$register = a;
            Ok(())
        }

        handler!($handler, $body);
    };
}

macro_rules! unary_op_handler {
    ( $handler:ident, $body:ident, $op:tt ) => {
        fn $body(vm: &mut VM) -> Result<()> {
            let x = vm.pop()?;
            vm.push($op x)?;
            Ok(())
        }

        handler!($handler, $body);
    };
}

bin_op_handler!(AddHandler, add_handler_body, +);
bin_op_handler!(SubHandler, sub_handler_body, -);
bin_op_handler!(BitwiseAndHandler, bitwise_and_handler_body, &);
bin_op_handler!(BitwiseOrHandler, bitwise_or_handler_body, |);
bin_op_handler!(BitwiseXorHandler, bitwise_xor_handler_body, ^);
bin_op_handler!(LeftShiftHandler, left_shift_handler_body, <<);
bin_op_handler!(RightShiftHandler, right_shift_handler_body, >>);
bin_op_handler!(MulHandler, mul_handler_body, *);
bin_op_handler!(DivHandler, div_handler_body, /);
bin_op_handler!(ModHandler, mod_handler_body, %);

unary_op_handler!(NegHandler, neg_handler_body, -);
unary_op_handler!(BitwiseNotHandler, bitwise_not_handler_body, !);

get_register_handler!(GetIPHandler, get_ip_handler_body, ip);
get_register_handler!(GetFPHandler, get_fp_handler_body, fp);
get_register_handler!(GetSPHandler, get_sp_handler_body, sp);
get_register_handler!(GetRVHandler, get_rv_handler_body, rv);

set_register_handler!(SetIPHandler, set_ip_handler_body, ip);
set_register_handler!(SetFPHandler, set_fp_handler_body, fp);
set_register_handler!(SetSPHandler, set_sp_handler_body, sp);
set_register_handler!(SetRVHandler, set_rv_handler_body, rv);

pub struct HaltHandler;
impl CommandHandler for HaltHandler {
    fn handle(&self, vm: &mut VM, _: &dyn InputOutput) -> Result<Option<ReturnCode>> {
        let rc = vm.pop()?;
        Ok(Some(rc))
    }
}

pub struct InHandler;
impl CommandHandler for InHandler {
    fn handle(&self, vm: &mut VM, io: &dyn InputOutput) -> Result<Option<ReturnCode>> {
        let c = io.get_char()?;
        vm.push(c)?;
        Ok(None)
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

fn cmp_handler_body(vm: &mut VM) -> Result<()> {
    let y = vm.pop()?;
    let x = vm.pop()?;
    if x < y {
        vm.push(-1)?;
    } else if x > y {
        vm.push(1)?;
    } else {
        vm.push(0)?;
    }
    Ok(())
}
handler!(CmpHandler, cmp_handler_body);


fn dup_handler_body(vm: &mut VM) -> Result<()> {
    let x = vm.pop()?;
    vm.push(x)?;
    vm.push(x)?;
    Ok(())
}
handler!(DupHandler, dup_handler_body);

fn drop_handler_body(vm: &mut VM) -> Result<()> {
    let _ = vm.pop()?;
    Ok(())
}
handler!(DropHandler, drop_handler_body);

fn swap_handler_body(vm: &mut VM) -> Result<()> {
    let y = vm.pop()?;
    let x = vm.pop()?;
    vm.push(y)?;
    vm.push(x)?;
    Ok(())
}
handler!(SwapHandler, swap_handler_body);

fn rot_handler_body(vm: &mut VM) -> Result<()> {
    let z = vm.pop()?;
    let y = vm.pop()?;
    let x = vm.pop()?;
    vm.push(y)?;
    vm.push(z)?;
    vm.push(x)?;
    Ok(())
}
handler!(RotHandler, rot_handler_body);

fn over_handler_body(vm: &mut VM) -> Result<()> {
    let y = vm.pop()?;
    let x = vm.pop()?;
    vm.push(x)?;
    vm.push(y)?;
    vm.push(x)?;
    Ok(())
}
handler!(OverHandler, over_handler_body);

fn sdrop_handler_body(vm: &mut VM) -> Result<()> {
    let y = vm.pop()?;
    let _ = vm.pop()?;
    vm.push(y)?;
    Ok(())
}
handler!(SdropHandler, sdrop_handler_body);

fn drop2_handler_body(vm: &mut VM) -> Result<()> {
    let _ = vm.pop()?;
    let _ = vm.pop()?;
    Ok(())
}
handler!(Drop2Handler, drop2_handler_body);

fn load_handler_body(vm: &mut VM) -> Result<()> {
    let address = vm.pop()?;
    let data = vm.read_memory(address)?;
    vm.push(data)?;
    Ok(())
}
handler!(LoadHandler, load_handler_body);

fn save_handler_body(vm: &mut VM) -> Result<()> {
    let value = vm.pop()?;
    let address = vm.pop()?;
    vm.write_memory(address, value)?;
    Ok(())
}
handler!(SaveHandler, save_handler_body);

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
