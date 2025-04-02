use anyhow::Result;

use crate::models::vm::VM;
use crate::models::command::ReturnCode;

use super::command::get_handler;

pub struct Executor<'a, IO: Input + Output> {
    io: &'a IO,
}

impl<'a, IO: Input + Output> Executor<'a, IO> {
    pub fn execute(&self, mut vm: VM) -> Result<ReturnCode> {
        loop {
            if let Some(rc) = self.execute_step(&mut vm)? {
                return Ok(rc)
            }
        }
    }

    fn execute_step(&self, vm: &mut VM) -> Result<Option<ReturnCode>> {
        let ip = vm.registers().ip;
        let opcode = vm.read_memory(ip)?;
        vm.registers_mut().ip += 1;
        match opcode {
            0.. => {
                vm.push(opcode)?;
                Ok(None)
            },
            ..=-1 => get_handler(opcode)?.handle(vm)
        }
    }
}

pub trait Input {
    fn get_char(&self) -> i64;
}

pub trait Output {
    fn print_char(c: i64);
}

#[cfg(test)]
mod tests {
    use crate::logic::{assembly::{self, TextFile}, stdio::Stdio};

    use super::*;

    #[test]
    fn halt_returns_error_code() {
        let files = &[TextFile{name: "stdin".to_string(), text: "2 3 ADD HALT".to_string()}];
        let instructions = assembly::assembly(files).unwrap();
        let vm = VM::new(instructions);
        let io = &Stdio{};
        let executor = Executor{io};

        let rc = executor.execute(vm).unwrap();

        assert_eq!(rc, 5)
    }

    #[test]
    fn executes_simple_program() {
        let files = &[TextFile{name: "stdin".to_string(), text: "2 3 ADD 0 HALT".to_string()}];
        let instructions = assembly::assembly(files).unwrap();
        let vm = VM::new(instructions);
        let io = &Stdio{};
        let executor = Executor{io};

        let rc = executor.execute(vm).unwrap();

        assert_eq!(rc, 0)
    }

    #[test]
    fn halt_on_empty_stack_fails() {
        let files = &[TextFile{name: "stdin".to_string(), text: "HALT".to_string()}];
        let instructions = assembly::assembly(files).unwrap();
        let vm = VM::new(instructions);
        let io = &Stdio{};
        let executor = Executor{io};

        let got = executor.execute(vm);

        assert_eq!(got.unwrap_err().to_string(), "invalid memory read at 1000000")
    }

    #[test]
    fn out_instruction_outputs_symbol() {
        let files = &[TextFile{name: "stdin".to_string(), text: "2 3 ADD OUT 0 HALT".to_string()}];
        let instructions = assembly::assembly(files).unwrap();
        let vm = VM::new(instructions);
        let io = &Stdio{};
        let executor = Executor{io};

        let _ = executor.execute(vm).unwrap();


    }
}
