use crate::reg::Regs;

pub const MEM_SIZE: usize = 256;

enum Opcode {
    MOV { dest: u8, src: u8, val: Option<u8> },
    ADD,
    HLT
}

impl Opcode {
    pub fn match_byte(cpu: &mut Cpu) -> Option<Self>{
        let byte = cpu.fetch_next_byte();

        match byte {
            0x00 => Some(Opcode::HLT),
            0x01 => Some(Opcode::ADD),
            0x02 => {
                todo!("MOV matcher");
            }
            _ => None
        }
    }

    pub fn exec(&self, cpu: &mut Cpu) {
        match self {
            Opcode::HLT => {
                // TODO: trace
                cpu.state = false;
            }
            Opcode::ADD => {
                // TODO: add instraction with args
                // TODO: trace
                cpu.regs.a = cpu.regs.a + cpu.regs.b;
            }
            Opcode::MOV { dest, src, val } => {
                todo!("MOV implementation");
            }
        }
    }
}

pub struct Cpu {
    pub regs: Regs,
    pub pc: u16,
    pub mem: [u8; MEM_SIZE],
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
        for (i, &byte) in data.iter().enumerate() {
            self.mem[i] = byte;
        }
        self.pc = 0;
    }

    pub fn fetch_next_byte(&mut self) -> u8 {
        let byte = self.mem[self.pc as usize];
        self.pc += 1;
        byte
    }

    pub fn run(&mut self) {
        while self.state {
            if !self.state { return; }

            if let Some(opcode) = Opcode::match_byte(self) {
                opcode.exec(self);
            } else {
                self.state = false;
                println!("Unknown operation");
            }
        }
    }
}
