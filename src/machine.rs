use crate::{TRACE, cpu::*, opcode::Opcode, reg::REG_A, reg::REG_B, reg::REG_C};

mod conditional_flags {
    pub const EQ: u8 = 1 << 0;      // Equal
    pub const NE: u8 = 1 << 1;      // Not Equal
    pub const GT: u8 = 1 << 2;      // Greater Than
    pub const LT: u8 = 1 << 3;      // Less Than
    pub const GE: u8 = 1 << 4;      // Greater or Equal
    pub const LE: u8 = 1 << 5;      // Less or Equal
    pub const ZE: u8 = 1 << 6;      // Zero
    pub const NZ: u8 = 1 << 7;      // Not Zero
}

pub struct Machine {
    trace: bool,
    cpu: Cpu
}

impl Machine {
    pub fn create() -> Self {
        Machine {
            trace: TRACE,
            cpu: Cpu::create(),
        }
    }

    pub fn load_program(&mut self, program: &[u8]) {
        self.cpu.load_prog(program);
        self.trace(&format!("TRACE: Loaded {} instructions", program.len()));
    }

    pub fn run(&mut self) {
        self.trace("TRACE: Starting execution");

        while self.cpu.is_running() {
            if let Some(op) = Opcode::decode(&mut self.cpu) {
                self.exec( &op);
            } else {
                eprintln!("ERROR: Failed to decode instruction at PC 0x{:04X}", self.cpu.get_pc());
                self.cpu.halt();
            }
        }
    }

    fn exec(&mut self, opcode: &Opcode) {
        match *opcode {
            Opcode::HLT => self.exec_halt(),
            Opcode::ADD => self.exec_add(),
            Opcode::SUB => self.exec_sub(),
            Opcode::MUL => self.exec_mul(),
            Opcode::JMP(address) => self.exec_jmp(address),
            Opcode::CMP => self.exec_cmp(),
            Opcode::JCT { mask, address } => self.exec_jct(mask, address),
            Opcode::INC(address) => self.exec_inc(address),
            Opcode::DEC(address) => self.exec_dec(address),
            Opcode::OFS(offset) => self.exec_ofs(offset),
            Opcode::JOR { mask, address } => self.exec_jor(mask, address),
            Opcode::NOP => self.trace("NOP: nothing to do"),
            Opcode::STR { reg, address } => self.exec_str(reg, address),
            Opcode::LMR { reg, address } => self.exec_lmr(reg, address),
        }

    }

    fn trace(&self, ms: &str) {
        if self.trace {
            println!("TRACE: {}", ms);
        }
    }

    fn exec_halt(&mut self) {
        self.trace("TRACE: HLT");
        self.cpu.halt();
    }

    fn exec_add(&mut self) {
        let a = self.cpu.read_reg(REG_A).expect("ERROR: [ADD] invalid register 'a' ID");
        let b = self.cpu.read_reg(REG_B).expect("ERROR: [ADD] invalid register 'b' ID");
        self.trace(&format!("TRACE: ADD, A = {}, B = {}", a, b));
        if !self.cpu.write_reg(REG_C, a + b) {
            eprintln!("ERROR: cannot perform ADD");
            self.cpu.halt();
        }
    }

    fn exec_sub(&mut self) {
        let a = self.cpu.read_reg(REG_A).expect("ERROR: [SUB] invalid register 'a' ID");
        let b = self.cpu.read_reg(REG_B).expect("ERROR: [SUB] invalid register 'b' ID");
        self.trace(&format!("TRACE: SUB, A = {}, B = {}", a, b));
        if !self.cpu.write_reg(REG_C, b - a) {
            eprintln!("ERROR: cannot perform ADD");
            self.cpu.halt();
        }
    }

    fn exec_mul(&mut self) {
        let a = self.cpu.read_reg(REG_A).expect("ERROR: [MUL] invalid register 'a' ID");
        let b = self.cpu.read_reg(REG_B).expect("ERROR: [MUL] invalid register 'b' ID");
        self.trace(&format!("TRACE: MUL, A = {}, B = {}", a, b));
        if !self.cpu.write_reg(REG_C, a * b) {
            eprintln!("ERROR: cannot perform ADD");
            self.cpu.halt();
        }
    }

    fn exec_jmp(&mut self, address: u16) {
        self.trace(&format!("TRACE: JMP, ADDRESS = {}", address));
        self.cpu.set_pc(address);
    }

    fn exec_cmp(&mut self) {
        let a = self.cpu.read_reg(REG_A).expect("ERROR: [CMP] invalid register 'a' ID");
        let b = self.cpu.read_reg(REG_B).expect("ERROR: [CMP] invalid register 'b' ID");
        let f = ((a == b) as u8 * conditional_flags::EQ) |
                   ((a != b) as u8 * conditional_flags::NE) |
                   ((a > b) as u8 * conditional_flags::GT)  |
                   ((a < b) as u8 * conditional_flags::LT)  |
                   ((a >= b) as u8 * conditional_flags::GE) |
                   ((a <= b) as u8 * conditional_flags::LE) |
                   ((a == 0) as u8 * conditional_flags::ZE) |
                   ((a != 0) as u8 * conditional_flags::NZ);
        self.trace(&format!("TRACE: CMP, A = {}, B = {}, F = {:#b}", a, b, f));
        self.cpu.set_flags(f);
    }

    fn exec_jct(&mut self, mask: u8, address: u16) {
        let flags = self.cpu.get_flags();
        let ok: bool = (flags & mask) == 0;
        self.trace(&format!("TRACE: JCT, mask=0b{:08b}, F=0b{:08b} -> {}", mask, flags, if ok { "TAKEN" } else { "NOT TAKEN" }));
        if ok { self.cpu.set_pc(address); }
    }

    fn exec_inc(&mut self, address: u8) {
        // TODO: trace
        let value = self.cpu.read_reg(address).expect("ERROR: [INC] invalid register ID");
        if !self.cpu.write_reg(address, value + 1) {
            eprintln!("ERROR: cannot perform INC on register {}", address);
            self.cpu.halt();
        }
    }

    fn exec_dec(&mut self, address: u8) {
        // TODO: trace
        let value = self.cpu.read_reg(address).expect("ERROR: [DEC] invalid register ID");
        if !self.cpu.write_reg(address, value - 1) {
            eprintln!("ERROR: cannot perform DEC on register {}", address);
            self.cpu.halt();
        }
    }

    fn exec_ofs(&mut self, offset: u16) {
        // TODO: trace
        self.cpu.set_pc(self.cpu.get_pc() + offset);
    }

    fn exec_jor(&mut self, mask: u8, address: u16) {
        if (mask == 0) || ((mask & (mask - 1)) != 0) {
            self.cpu.halt();
            eprintln!("ERROR: JOR requires exactly one bit in mask (got 0b{:08b})", mask);
            return;
        }
        let flags = self.cpu.get_flags();
        let ok: bool = (flags & mask) != 0;
        self.trace(&format!("TRACE: JOR, mask=0b{:08b}, F=0b{:08b} -> {}", mask, flags, if ok { "TAKEN" } else { "NOT TAKEN" }));
        if ok { self.cpu.set_pc(address); }
    }

    fn exec_str(&mut self, register_addr: u8, mem_addr: u16) {
        let value = self.cpu.read_reg(register_addr).expect("ERROR: [STR] invalid register ID");
        if !self.cpu.write_mem(mem_addr, value) {
            eprintln!("ERROR: cannot perform STR to address 0x{:04X}", mem_addr);
            self.cpu.halt();
        }
        self.trace(&format!("TRACE: STR, mem[0x{:04X}] <= {} ({})", mem_addr, match register_addr { 0 => "A", 1 => "B", 2 => "C", _ => "?" }, value));
    }

    fn exec_lmr(&mut self, register_addr: u8, mem_addr: u16) {
        let value = self.cpu.read_mem(mem_addr).expect("ERROR: [LMR] invalid memory address");
        if !self.cpu.write_reg(register_addr, value) {
            eprintln!("ERROR: cannot perform LMR from address 0x{:04X}", mem_addr);
            self.cpu.halt();
        }
        self.trace(&format!("TRACE: LMR, mem[0x{:04X}] => {} ({})", mem_addr, match register_addr { 0 => "A", 1 => "B", 2 => "C", _ => "?" }, value));
    }
}

// Opcode::MOV { mode, dest, src } => {
//     // TODO: trace
//     let source_val = match mode {
//         Some(REG_TO_REG_MOV_MODE) => {
//             match *src {
//                 REG_A => cpu.regs.a,
//                 REG_B => cpu.regs.b,
//                 REG_C => cpu.regs.c,
//                 _ => {
//                     cpu.state = false;
//                     eprintln!("ERROR: src ID is incorrect {}", src);
//                     return;
//                 }
//             }
//         },
//         Some(CONST_TO_REG_MOV_MODE) => {
//             *src
//         },
//         _ => {
//             cpu.state = false;
//             eprintln!("ERROR: MOV mode is incorrect");
//             return;
//         }
//     };
//     match *dest {
//         REG_A => cpu.regs.a = source_val,
//         REG_B => cpu.regs.b = source_val,
//         REG_C => cpu.regs.c = source_val,
//         _ => {
//             cpu.state = false;
//             eprintln!("ERROR: dest ID is incorrect {}", src);
//         },
//     }
// },
