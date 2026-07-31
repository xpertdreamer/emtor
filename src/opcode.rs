use crate::cpu::*;

// MOV_OPCODE => {
            //     let mode = cpu.fetch_next_byte();
            //     match mode {
            //         // From reg to reg
            //         REG_TO_REG_MOV_MODE => {
            //             let dest = cpu.fetch_next_byte();
            //             let src = cpu.fetch_next_byte();
            //             Some(Opcode::MOV { mode: Some(REG_TO_REG_MOV_MODE), dest, src })
            //         },
            //         // Const to reg
            //         CONST_TO_REG_MOV_MODE => {
            //             let dest = cpu.fetch_next_byte();
            //             let value = cpu.fetch_next_byte();
            //             Some(Opcode::MOV { mode: Some(CONST_TO_REG_MOV_MODE), dest, src: value })
            //         },
            //         _ => None
            //     }
            // },

mod opcodes {
    pub const HLT: u8 = 0x00;
    pub const ADD: u8 = 0x01;
    // pub const MOV_OPCODE: u8 = 0x02;
    pub const SUB: u8 = 0x03;
    pub const MUL: u8 = 0x04;
    pub const JMP: u8 = 0x05;
    pub const CMP: u8 = 0x06;
    pub const JCT: u8 = 0x07;
    pub const INC: u8 = 0x08;
    pub const DEC: u8 = 0x09;
    pub const OFS: u8 = 0x0A;
    pub const JOR: u8 = 0x0B;
    pub const NOP: u8 = 0x0C;
    pub const STR: u8 = 0x0D;
    pub const LMR: u8 = 0x0E;
}

#[allow(clippy::upper_case_acronyms)]
pub enum Opcode {
    // TODO: replace mode with arm style, or create separate instruction
    // MOV { mode: Option<u8>, dest: u8, src: u8 },
    ADD,
    HLT,
    SUB,
    MUL,
    JMP(u16),
    CMP,
    JCT{ mask: u8, address: u16 },
    INC(u8),
    DEC(u8),
    OFS(u16),
    JOR{ mask: u8, address: u16 },
    NOP,
    STR {reg: u8, address: u16 },
    LMR {reg: u8, address: u16}
}

impl Opcode {
    fn high_end(cpu: &mut Cpu) -> u16 {
        let h: u8 = cpu.fetch_next_byte().expect("ERROR: Memory out of bound, while reading u16");
        let l: u8 = cpu.fetch_next_byte().expect("ERROR: Memory out of bound, while reading u16");
        let address: u16 = ((h as u16) << 8) | l as u16;
        address
    }

    pub fn decode(cpu: &mut Cpu) -> Option<Self>{
        let byte = cpu.fetch_next_byte().expect("ERROR: Memory out of bound, while reading u16");

        match byte {
            opcodes::HLT => Some(Opcode::HLT),
            opcodes::ADD => Some(Opcode::ADD),
            opcodes::SUB => Some(Opcode::SUB),
            opcodes::MUL => Some(Opcode::MUL),
            opcodes::CMP => Some(Opcode::CMP),
            opcodes::NOP => Some(Opcode::NOP),
            opcodes::JMP =>
            {
                let address = Self::high_end(cpu);
                Some(Opcode::JMP(address))
            },
            opcodes::JCT => {
                let mask = cpu.fetch_next_byte().expect("ERROR: Memory out of bound, while reading u16");
                let address = Self::high_end(cpu);
                Some(Opcode::JCT { mask, address })
            },
            opcodes::INC => {
                let address = cpu.fetch_next_byte().expect("ERROR: Memory out of bound, while reading u16");
                Some(Opcode::INC(address))
            },
            opcodes::DEC => {
                let address = cpu.fetch_next_byte().expect("ERROR: Memory out of bound, while reading u16");
                Some(Opcode::DEC(address))
            },
            opcodes::OFS => {
                let offset = Self::high_end(cpu);
                Some(Opcode::OFS(offset))
            },
            opcodes::JOR => {
                let mask = cpu.fetch_next_byte().expect("ERROR: Memory out of bound, while reading u16");
                let address = Self::high_end(cpu);
                Some(Opcode::JOR { mask, address })
            },
            opcodes::STR=> {
                let reg = cpu.fetch_next_byte().expect("ERROR: Memory out of bound, while reading u16");
                let address = Self::high_end(cpu);
                Some(Opcode::STR { reg, address })
            },
            opcodes::LMR => {
                let reg = cpu.fetch_next_byte().expect("ERROR: Memory out of bound, while reading u16");
                let address = Self::high_end(cpu);
                Some(Opcode::LMR { reg, address })
            }
            _ => None
        }
    }
}
