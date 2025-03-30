use super::command::Instruction;

#[derive(Debug, PartialEq)]
pub struct Registers {
    pub ip: i64,
    pub sp: i64,
    pub fp: i64,
    pub rv: i64,
}

pub const MEM_SIZE: i64 = 1000000;
pub type Memory = Vec<i64>;

pub struct VM {
    registers: Registers,
    memory: Memory,
    code: Vec<Instruction>,
}

impl VM {
    pub fn new(code: Vec<Instruction>) -> Self {
        let res = Self {
            memory: vec![0; MEM_SIZE as usize].try_into().unwrap(),
            registers: Registers{
                ip: 256,
                sp: MEM_SIZE,
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
}
