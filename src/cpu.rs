use crate::{TRACE, reg::*};

pub const MEM_SIZE: usize = 256;

// MOV MODES
pub const REG_TO_REG_MOV_MODE: u8 = 0x0A;
pub const CONST_TO_REG_MOV_MODE: u8 = 0x0B;

// OPCODES
pub const HLT_OPCODE: u8 = 0x00;
pub const ADD_OPCODE: u8 = 0x01;
pub const MOV_OPCODE: u8 = 0x02;
pub const SUB_OPCODE: u8 = 0x03;
pub const MUL_OPCODE: u8 = 0x04;
pub const JMP_OPCODE: u8 = 0x05;

enum Opcode {
    MOV { mode: Option<u8>, dest: u8, src: u8 },
    ADD,
    HLT,
    SUB,
    MUL,
    JMP(u16)
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
            },
            SUB_OPCODE => Some(Opcode::SUB),
            MUL_OPCODE => Some(Opcode::MUL),
            JMP_OPCODE =>
            {
                let high = cpu.fetch_next_byte();
                let low = cpu.fetch_next_byte();
                let address: u16 = ((high as u16) << 8) | low as u16;
                Some(Opcode::JMP(address))
            },
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
                if TRACE { println!("TRACE: EXECUTING ADD, A = {}, B = {}", cpu.regs.a, cpu.regs.b ); }
                cpu.regs.a = cpu.regs.a + cpu.regs.b;
            }
            Opcode::MOV { mode, dest, src } => {
                // TODO: trace
                let source_val = match mode {
                    Some(REG_TO_REG_MOV_MODE) => {
                        match src {
                            &REG_A => cpu.regs.a,
                            &REG_B => cpu.regs.b,
                            _ => {
                                cpu.state = false;
                                eprintln!("ERROR: src ID is incorrect {}", src);
                                return;
                            }
                        }
                    }
                    Some(CONST_TO_REG_MOV_MODE) => {
                        *src
                    }
                    _ => {
                        cpu.state = false;
                        eprintln!("ERROR: MOV mode is incorrect");
                        return;
                    }
                };

                match *dest {
                    REG_A => cpu.regs.a = source_val,
                    REG_B => cpu.regs.b = source_val,
                    _ => {
                        cpu.state = false;
                        eprintln!("ERROR: dest ID is incorrect {}", dest);
                    }
                }
            },
            Opcode::SUB => {
                if TRACE { println!("TRACE: EXECUTING SUB, A = {}, B = {}", cpu.regs.a, cpu.regs.b ); }
                cpu.regs.a = cpu.regs.a - cpu.regs.b;
            },
            Opcode::MUL => {
                if TRACE { println!("TRACE: EXECUTING MUL, A = {}, B = {}", cpu.regs.a, cpu.regs.b ); }
                cpu.regs.a = cpu.regs.a * cpu.regs.b;
            },
            Opcode::JMP(address) => {
                if TRACE { println!("TRACE: EXECUTING JMP, ADDRESS = {}", address ); }
                cpu.pc = *address;
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
        if (self.pc as usize) < MEM_SIZE {
            let byte = self.mem[self.pc as usize];
            self.pc += 1;
            return byte;
        } else {
            self.state = false;
            eprintln!("ERROR: End of memory");
            return HLT_OPCODE;
        }
    }

    pub fn run(&mut self) {
        while self.state {
            if !self.state { return; }

            if let Some(opcode) = Opcode::match_byte(self) {
                opcode.exec(self);
            } else {
                self.state = false;
                eprintln!("Unknown operation");
            }
        }
    }
}
