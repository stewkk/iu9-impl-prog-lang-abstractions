use anyhow::Result;

use crate::models::vm::VM;

use super::command::get_handler;

pub fn execute(mut vm: VM) -> Result<()> {
    execute_step(&mut vm)
}

fn execute_step(vm: &mut VM) -> Result<()> {
    let ip = vm.registers().ip;
    let opcode = vm.read_memory(ip)?;
    vm.registers_mut().ip += 1;
    match opcode {
        0.. => vm.push(opcode),
        ..=-1 => get_handler(opcode)?.handle(vm),
    }
}

#[cfg(test)]
mod tests {
    use crate::logic::assembly::{self, TextFile};

    use super::*;

    #[test]
    fn executes_simple_program() {
        let files = &[TextFile{name: "stdin".to_string(), text: "2 3 ADD 0 HALT".to_string()}];
        let instructions = assembly::assembly(files).unwrap();
        let vm = VM::new(instructions);

        execute(vm).unwrap();

        // TODO: asserts
    }
}
