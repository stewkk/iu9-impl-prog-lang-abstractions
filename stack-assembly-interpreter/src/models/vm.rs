#[derive(Debug, PartialEq)]
pub struct Registers {
    pub ip: i64,
    pub sp: i64,
    pub fp: i64,
    pub rv: i64,
}

pub const MEM_SIZE: i64 = 1000000;
pub type Memory = Box<[i64; MEM_SIZE as usize]>;

pub struct VM {
    registers: Registers,
    memory: Memory,
}

impl VM {
    pub fn new() -> Self {
        let res = Self {
            memory: vec![0; MEM_SIZE as usize].try_into().unwrap(),
            registers: Registers{
                ip: 256,
                sp: MEM_SIZE,
                fp: 0,
                rv: 0,
            },
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
        let vm = VM::new();

        let got = vm.registers();

        assert_eq!(got, &Registers{
            ip: 256,
            sp: 1000000,
            fp: 0,
            rv: 0,
        })
    }
}
