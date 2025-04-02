use anyhow::Result;

use crate::models::vm::VM;

pub fn execute(vm: VM) -> Result<()> {
    Ok(())
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
