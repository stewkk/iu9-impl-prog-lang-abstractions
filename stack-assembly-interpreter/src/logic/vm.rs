use anyhow::{anyhow, Context, Result, Error};

use crate::models::token::Token;
use crate::models::vm::VM;
use crate::models::command::{Input, Instruction, Output, ReturnCode};

use super::command::get_handler;

// NOTE: it's easier here to use a crate that can create mock of struct
pub struct Executor<'a, IO: Input + Output> {
    pub io: &'a mut IO,
}

impl<'a, IO: Input + Output> Executor<'a, IO> {
    pub fn execute(&mut self, mut vm: VM) -> Result<ReturnCode> {
        loop {
            if let Some(rc) = self.execute_step(&mut vm)? {
                return Ok(rc)
            }
        }
    }

    fn execute_step(&mut self, vm: &mut VM) -> Result<Option<ReturnCode>> {
        let ip = vm.registers().ip;
        let instruction = vm.read_code(ip)?.clone();
        let opcode = instruction.opcode;
        vm.registers_mut().ip += 1;
        (|| match opcode {
            0.. => {
                vm.push(opcode)?;
                Ok(None)
            },
            ..=-1 => get_handler(opcode)?.handle(vm, self.io)
        })().context(get_failed_to_execute_error(&instruction))
    }
}

fn get_failed_to_execute_error(instruction: &Instruction) -> Error {
    match &instruction.token {
        Token::Integer(i, pos) => anyhow!("{pos}: failed to execute integer instruction {i}"),
        Token::Declaration(i, pos) => anyhow!("{pos}: can't execute declaration {i}"),
        Token::Ident(i, pos) => anyhow!("{pos}: failed to execute ident instruction {i}"),
    }
}

#[cfg(test)]
mod tests {
    use mockall::{mock, predicate};

    use crate::logic::{assembly::{self, TextFile}, stdio::Stdio};

    use super::*;

    #[test]
    fn halt_returns_error_code() {
        let files = &[TextFile{name: "stdin".to_string(), text: "2 3 ADD HALT".to_string()}];
        let instructions = assembly::assembly(files).unwrap();
        let vm = VM::new(instructions);
        let mut io = Stdio::new();
        let mut executor = Executor{io: &mut io};
        let rc = executor.execute(vm).unwrap();

        assert_eq!(rc, 5)
    }

    #[test]
    fn executes_simple_program() {
        let files = &[TextFile{name: "stdin".to_string(), text: "2 3 ADD 0 HALT".to_string()}];
        let instructions = assembly::assembly(files).unwrap();
        let vm = VM::new(instructions);
        let mut io = Stdio::new();
        let mut executor = Executor{io: &mut io};

        let rc = executor.execute(vm).unwrap();

        assert_eq!(rc, 0)
    }

    #[test]
    fn halt_on_empty_stack_fails() {
        let files = &[TextFile{name: "stdin".to_string(), text: "HALT".to_string()}];
        let instructions = assembly::assembly(files).unwrap();
        let vm = VM::new(instructions);
        let mut io = Stdio::new();
        let mut executor = Executor{io: &mut io};

        let got = executor.execute(vm);

        assert_eq!(got.unwrap_err().to_string(), "stdin:1:1: failed to execute ident instruction HALT")
    }

    mock! {
        InputOutput {}
        impl Input for InputOutput {
            fn get_char(&mut self) -> Result<i64>;
        }
        impl Output for InputOutput {
            fn print_char(&self, c: i64) -> Result<()>;
        }
    }

    #[test]
    fn out_instruction_outputs_symbol() {
        let files = &[TextFile{name: "stdin".to_string(), text: "2 3 ADD OUT 0 HALT".to_string()}];
        let instructions = assembly::assembly(files).unwrap();
        let vm = VM::new(instructions);
        let mut io = MockInputOutput::new();

        io.expect_print_char()
          .with(predicate::eq(5))
          .return_once(|_| Ok(()));

        let mut executor = Executor{io: &mut io};
        let _ = executor.execute(vm).unwrap();
    }
}
