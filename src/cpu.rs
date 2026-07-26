use crate::reg::Regs;

const MEM_SIZE: usize = 256;

pub struct Cpu {
    regs: Regs,
    pc: u16,
    mem: [u8; MEM_SIZE],
    state: bool
}

impl Cpu {
    pub fn create() -> Self {
        Cpu {
            regs: Regs::create(),
            pc: 0,
            mem: [0; MEM_SIZE],
            state: true
        }
    }
}
