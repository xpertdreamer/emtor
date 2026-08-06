use crate::cpu::*;

mod opcodes {
    pub const HLT: u8 = 0x00;
    pub const ADD: u8 = 0x01;
    pub const MOV: u8 = 0x02;
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
    pub const MOC: u8 = 0x0F;
    pub const NOT: u8 = 0x10;
    pub const XOR: u8 = 0x11;
    pub const BOR: u8 = 0x12;
    pub const AND: u8 = 0x13;
    pub const JOF: u8 = 0x14;
    pub const PSH: u8 = 0x15;
    pub const POP: u8 = 0x16;
    pub const MOD: u8 = 0x17;
    pub const IWG: u8 = 0x18;
//    pub const GMB: u8 = 0x19;
}

#[allow(clippy::upper_case_acronyms)]
pub enum Opcode {
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
    LMR {reg: u8, address: u16},
    MOV { dest: u8, src: u8 },
    MOC { dest: u8, value: u8 },
    NOT(u8),
    XOR {dest: u8, src: u8 },
    BOR {dest: u8, src: u8 },
    AND {dest: u8, src: u8 },
    JOF(u16),
    PSH(u8),
    POP(u8),
    MOD,
    IWG(u16),
//    GMB(u16)
}

impl Opcode {
    fn high_end(cpu: &mut Cpu) -> u16 {
        let h: u8 = cpu.fetch_next_byte().expect("ERROR: Memory out of bound, while reading u16");
        let l: u8 = cpu.fetch_next_byte().expect("ERROR: Memory out of bound, while reading u16");
        let address: u16 = ((h as u16) << 8) | l as u16;
        address
    }

    fn high_end_st(cpu: &mut Cpu) -> u16 {
        let h: u8 = cpu.pop().expect("ERROR: Stack overflow while reading u16 (h)");
        let l: u8 = cpu.pop().expect("ERROR: Stack overflow while reading u16 (l)");
        let address: u16 = ((h as u16) << 8) | l as u16;
        address
    }

    pub fn decode(cpu: &mut Cpu) -> Option<Self> {
        let byte = cpu.fetch_next_byte().expect("ERROR: Memory out of bound, while reading byte");
        match byte {
            opcodes::HLT => Some(Opcode::HLT),
            opcodes::ADD => Some(Opcode::ADD),
            opcodes::SUB => Some(Opcode::SUB),
            opcodes::MUL => Some(Opcode::MUL),
            opcodes::CMP => Some(Opcode::CMP),
            opcodes::NOP => Some(Opcode::NOP),
            opcodes::MOD => Some(Opcode::MOD),
            opcodes::JMP => {
                let address = Self::high_end(cpu);
                Some(Opcode::JMP(address))
            },
            opcodes::JCT => {
                let mask = cpu.fetch_next_byte().expect("ERROR: Memory out of bound");
                let address = Self::high_end(cpu);
                Some(Opcode::JCT { mask, address })
            },
            opcodes::INC => {
                let address = cpu.fetch_next_byte().expect("ERROR: Memory out of bound");
                Some(Opcode::INC(address))
            },
            opcodes::DEC => {
                let address = cpu.fetch_next_byte().expect("ERROR: Memory out of bound");
                Some(Opcode::DEC(address))
            },
            opcodes::OFS => {
                let offset = Self::high_end(cpu);
                Some(Opcode::OFS(offset))
            },
            opcodes::JOR => {
                let mask = cpu.fetch_next_byte().expect("ERROR: Memory out of bound");
                let address = Self::high_end(cpu);
                Some(Opcode::JOR { mask, address })
            },
            opcodes::STR=> {
                let reg = cpu.fetch_next_byte().expect("ERROR: Memory out of bound");
                let address = Self::high_end(cpu);
                Some(Opcode::STR { reg, address })
            },
            opcodes::LMR => {
                let reg = cpu.fetch_next_byte().expect("ERROR: Memory out of bound");
                let address = Self::high_end(cpu);
                Some(Opcode::LMR { reg, address })
            },
            opcodes::MOV => {
                let dest = cpu.fetch_next_byte().expect("ERROR: Decode MOV, Memory out of bounds (dest)");
                let src = cpu.fetch_next_byte().expect("ERROR: Decode MOV, Memory out of bounds (src)");
                Some(Opcode::MOV { dest, src })
            },
            opcodes::MOC => {
                let dest = cpu.fetch_next_byte().expect("ERROR: Decode MOC, Memory out of bounds (dest)");
                let value = cpu.fetch_next_byte().expect("ERROR: Decode MOC, Memory out of bounds (value)");
                Some(Opcode::MOC { dest, value })
            },
            opcodes::NOT => {
                let reg = cpu.fetch_next_byte().expect("ERROR: Decode NOT, Memory out of bounds (register)");
                Some(Opcode::NOT(reg))
            },
            opcodes::XOR => {
                let dest = cpu.fetch_next_byte().expect("ERROR: Decode XOR, Memory out of bounds (dest)");
                let src = cpu.fetch_next_byte().expect("ERROR: Decode XOR, Memory out of bounds (src)");
                Some(Opcode::XOR { dest, src })
            },
            opcodes::BOR => {
                let dest = cpu.fetch_next_byte().expect("ERROR: Decode BOR, Memory out of bounds (dest)");
                let src = cpu.fetch_next_byte().expect("ERROR: Decode BOR, Memory out of bounds (src)");
                Some(Opcode::BOR { dest, src })
            },
            opcodes::AND => {
                let dest = cpu.fetch_next_byte().expect("ERROR: Decode AND, Memory out of bounds (dest)");
                let src = cpu.fetch_next_byte().expect("ERROR: Decode AND, Memory out of bounds (src)");
                Some(Opcode::AND { dest, src })
            },
            opcodes::JOF => {
                let addr = Self::high_end(cpu);
                Some(Opcode::JOF(addr))
            },
            opcodes::PSH => {
                let src = cpu.fetch_next_byte().expect("ERROR: Decode PSH, Memory out of bounds (src)");
                Some(Opcode::PSH(src))
            },
            opcodes::POP => {
                let dest = cpu.fetch_next_byte().expect("ERROR: Decode POP, Memory out of bounds (dest)");
                Some(Opcode::POP(dest))
            },
            opcodes::IWG => {
                let addr = Self::high_end(cpu);
                Some(Opcode::IWG(addr))
            },
            // opcodes::GMB => {

            // },
            _ => None
        }
    }
}
