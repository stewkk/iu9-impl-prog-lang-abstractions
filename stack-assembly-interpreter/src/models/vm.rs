use anyhow::{bail, anyhow, Result, Context};

use super::command::Instruction;

#[derive(Debug, PartialEq)]
pub struct Registers {
    pub ip: i64,
    pub sp: i64,
    pub fp: i64,
    pub rv: i64,
}

pub type Memory = Vec<i64>;

pub struct VM {
    registers: Registers,
    memory: Memory,
    code: Vec<Instruction>,
}

impl VM {
    const MEM_SIZE: i64 = 1000*1000;
    const BANNED_SIZE: usize = 256;

    pub fn new(code: Vec<Instruction>) -> Self {
        let res = Self {
            memory: vec![0; (Self::MEM_SIZE as usize)-code.len()-Self::BANNED_SIZE],
            registers: Registers{
                ip: 256,
                sp: Self::MEM_SIZE,
                fp: 0,
                rv: 0,
            },
            code,
        };
        res
    }

    pub fn registers(&self) -> &Registers {
        &self.registers
    }

    pub fn registers_mut(&mut self) -> &mut Registers {
        &mut self.registers
    }

    pub fn read_memory(&self, i: i64) -> Result<i64> {
        (|| {
            if i < Self::BANNED_SIZE as i64 {
                bail!("addresses [-inf, 256) are banned");
            }
            Ok(0)
        })().context(anyhow!("invalid memory access at {i}"))
    }

    pub fn read_stack(&self, offset: i64) -> Result<i64> {
        self.read_memory(self.registers.sp + offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vm_in_initial_state_sets_registers() {
        let vm = VM::new(vec![]);

        let got = vm.registers();

        assert_eq!(got, &Registers{
            ip: 256,
            sp: 1000000,
            fp: 0,
            rv: 0,
        })
    }

    #[test]
    fn read_memory_error_on_first_256() {
        let vm = VM::new(vec![]);

        for i in -10..256 {
            assert_eq!(vm.read_memory(i).unwrap_err().to_string(), format!("invalid memory access at {i}"))
        }
    }
}
