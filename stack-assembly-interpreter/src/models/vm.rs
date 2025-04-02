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

enum InternalAddress {
    Code(usize),
    Memory(usize),
}

impl VM {
    const MEM_SIZE: i64 = 1000*1000;
    const BANNED_SIZE: usize = 256;

    pub fn new(code: Vec<Instruction>) -> Self {
        let actual_memory_size = (Self::MEM_SIZE as usize)-code.len()-Self::BANNED_SIZE;
        let res = Self {
            memory: vec![0; actual_memory_size],
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

    fn get_internal_address(&self, i: i64) -> Result<InternalAddress> {
        if i < Self::BANNED_SIZE as i64 {
            bail!("address range [-inf, 256) is forbidden to access")
        }
        let actual = usize::try_from(i)? - 256;
        if actual < self.code.len() {
            return Ok(InternalAddress::Code(actual))
        }
        let actual = actual - self.code.len();
        Ok(InternalAddress::Memory(actual))
    }

    pub fn read_memory(&self, i: i64) -> Result<i64> {
        (|| {
            match self.get_internal_address(i)? {
                InternalAddress::Code(internal) => Ok(self.code[internal].opcode),
                InternalAddress::Memory(internal) => self.memory.get(internal)
                                                                .copied()
                                                                .ok_or_else(|| anyhow!("address too big")),
            }
        })().context(anyhow!("invalid memory read at {i}"))
    }

    pub fn write_memory(&mut self, i: i64, data: i64) -> Result<()> {
        (|| {
            match self.get_internal_address(i)? {
                InternalAddress::Code(_) => bail!("attempt to write at code segment"),
                InternalAddress::Memory(internal) =>
                    Ok(*self.memory.get_mut(internal).ok_or_else(|| anyhow!("address too big"))? = data),
            }
        })().context(anyhow!("invalid memory write at {i}"))
    }

    pub fn read_stack(&self, offset: i64) -> Result<i64> {
        self.read_memory(self.registers.sp + offset).context("failed to read value from stack")
    }

    pub fn push(&mut self, data: i64) -> Result<()> {
        self.registers.sp -= 1;
        self.write_memory(self.registers.sp, data).context("failed to push value on stack")
    }

    pub fn pop(&mut self) -> Result<i64> {
        let res = self.read_stack(0)?;
        self.registers.sp += 1;
        Ok(res)
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
            assert_eq!(vm.read_memory(i).unwrap_err().to_string(), format!("invalid memory read at {i}"))
        }
    }
}
