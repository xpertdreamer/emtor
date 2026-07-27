use crate::reg::Regs;

pub const MEM_SIZE: usize = 256;

pub const REG_TO_REG_MOV_MODE: u8 = 0xA;
pub const CONST_TO_REG_MOV_MODE: u8 = 0xB;

pub const HLT_OPCODE: u8 = 0x00;
pub const ADD_OPCODE: u8 = 0x01;
pub const MOV_OPCODE: u8 = 0x02;

enum Opcode {
    MOV { mode: Option<u8>, dest: u8, src: u8 },
    ADD,
    HLT
}

impl Opcode {
    pub fn match_byte(cpu: &mut Cpu) -> Option<Self>{
        let byte = cpu.fetch_next_byte();

        match byte {
            HLT_OPCODE => Some(Opcode::HLT),
            ADD_OPCODE => Some(Opcode::ADD),
            MOV_OPCODE => {
                let mode = cpu.fetch_next_byte();
                match mode {
                    // From reg to reg
                    REG_TO_REG_MOV_MODE => {
                        let dest = cpu.fetch_next_byte();
                        let src = cpu.fetch_next_byte();
                        Some(Opcode::MOV { mode: Some(REG_TO_REG_MOV_MODE), dest, src })
                    }
                    // Const to reg
                    CONST_TO_REG_MOV_MODE => {
                        let dest = cpu.fetch_next_byte();
                        let value = cpu.fetch_next_byte();
                        Some(Opcode::MOV { mode: Some(CONST_TO_REG_MOV_MODE), dest, src: value })
                    }
                    _ => None
                }
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
            Opcode::MOV { mode, dest, src } => {
                let source_val = match mode {
                    Some(REG_TO_REG_MOV_MODE) => {
                        match src {
                            0 => cpu.regs.a,
                            1 => cpu.regs.b,
                            _ => {
                                cpu.state = false;
                                println!("ERROR: src ID is incorrect {}", src);
                                return;
                            }
                        }
                    }
                    Some(CONST_TO_REG_MOV_MODE) => {
                        *src
                    }
                    _ => {
                        cpu.state = false;
                        println!("ERROR: MOV mode is incorrect");
                        return;
                    }
                };

                match dest {
                    0 => cpu.regs.a = source_val,
                    1 => cpu.regs.b = source_val,
                    _ => {
                        cpu.state = false;
                        println!("ERROR: dest ID is incorrect {}", dest);
                    }
                }
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
