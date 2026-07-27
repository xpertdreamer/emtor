use crate::reg::Regs;

const MEM_SIZE: usize = 256;

// TODO: some collection or enum with opcodes
// Opcodes can be represented as hex values or just decimals for simplicity
enum Opcode {
    MOV { dest: u8, src: u8, val: Option<u8> },
    ADD,
    HALT
}

impl Opcode {
    pub fn match_byte(cpu: &mut Cpu) -> Option<Self>{
        let byte = cpu.fetch_next_byte();

        todo!("Design OPCODES");
        match byte {

            _ => None
        }
    }
}

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

    pub fn load_prog(&mut self, data: &[u8]) {
        todo!("Load program from slice for now");
    }

    pub fn fetch_next_byte(&mut self) -> u8 {
        let byte = self.mem[self.pc as usize];
        self.pc += 1;
        byte
    }
}
