use crate::{TRACE, cpu::*, opcode::Opcode, reg::{REG_A, REG_B, REG_C}};

#[derive(Debug, PartialEq)]
pub struct CpuState {
    pub pc: u16,
    pub sp: u16,
    pub csp: u16,
    pub flags: u8,
    pub sys_flags: u8,
    pub state: bool,
    pub reg_a: i8,
    pub reg_b: i8,
    pub reg_c: i8,
    pub mem: [u8; MEM_SIZE]
}

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

mod system_flags {
    pub const CF: u8 = 1 << 0;
    pub const OF: u8 = 1 << 1;
    #[allow(unused)]
    pub const SF: u8 = 1 << 2;
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

    pub fn dump(&self) -> CpuState {
        CpuState {
            pc: self.cpu.get_pc(),
            sp: self.cpu.get_sp(),
            csp: self.cpu.get_csp(),
            flags: self.cpu.get_flags(),
            sys_flags: self.cpu.get_sys_flags(),
            state: self.cpu.is_running(),
            reg_a: self.cpu.read_reg(REG_A).unwrap_or(0),
            reg_b: self.cpu.read_reg(REG_B).unwrap_or(0),
            reg_c: self.cpu.read_reg(REG_C).unwrap_or(0),
            mem: self.cpu.get_mem()
        }
    }

    pub fn load_program(&mut self, program: &[u8]) {
        self.cpu.load_prog(program);
        self.trace(&format!("Loaded {} lines", program.len()));
    }

    pub fn run(&mut self) {
        self.trace("Starting execution");

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
            Opcode::MOV { dest, src } => self.exec_mov(dest, src),
            Opcode::MOC { dest, value } => self.exec_moc(dest, value),
            Opcode::NOT(reg) => self.exec_not(reg),
            Opcode::XOR { dest, src } => self.exec_xor(dest, src),
            Opcode::BOR { dest, src } => self.exec_bor(dest, src),
            Opcode::AND { dest, src } => self.exec_and(dest, src),
            Opcode::JOF(address) => self.exec_jof(address),
            Opcode::PSH(src) => self.exec_psh(src),
            Opcode::POP(dest) => self.exec_pop(dest),
            Opcode::MOD => self.exec_mod(),
            Opcode::IWG(address) => self.exec_iwg(address),
            Opcode::GMB => self.exec_gmb(),
        }
    }

    fn trace(&self, ms: &str) {
        if self.trace {
            println!("TRACE: {}", ms);
        }
    }

    fn calc_sys_flags(&self, res: i8, cf: bool, of: bool) -> u8 {
        let mut result: u8 = 0;
        result |= system_flags::CF * cf as u8;
        result |= system_flags::OF * of as u8;
        result |= (res as u8) >> 7;
        result
    }

    // * * *
    //     ^
    // ^

    fn exec_gmb(&mut self) {
        //TODO: trace
        self.trace("GMB");
        let return_addr = match self.cpu.pop_call() {
            Some(addr) => addr,
            None => {
                eprintln!("ERROR: [GMB] call stack underflow");
                self.cpu.halt();
                return;
            }
        };
        self.cpu.set_pc(return_addr);
    }

    fn exec_iwg(&mut self, address: u16) {
        // TODO: trace
        self.trace("IWG");
        let ret = self.cpu.get_pc();
        if address >= CALL_STACK_START {
            eprintln!("ERROR: [IWG] invalid target address 0x{:04X}", address);
            self.cpu.halt();
            return;
        }
        if !self.cpu.push_call(ret) {
            eprintln!("ERROR: [IWG] call stack overflow");
            self.cpu.halt();
            return;
        }
        self.cpu.set_pc(address);
    }

    fn exec_mod(&mut self) {
        let a = self.cpu.read_reg(REG_A).expect("ERROR: [MOD] invalid register 'a' ID");
        let b = self.cpu.read_reg(REG_B).expect("ERROR: [MOD] invalid register 'b' ID");
        let res: i8 = a % b;
        if !self.cpu.write_reg(REG_C, res) {
            eprintln!("ERROR: cannot perform MOD");
            self.cpu.halt();
        }
        self.trace(&format!("MOD, A = {}, B = {}, RES = {}", a, b, res));
    }

    fn exec_psh(&mut self, src: u8) {
        let value = self.cpu.read_reg(src).expect("ERROR: [PSH] invalid src register ID");
        if !self.cpu.push(value) {
            eprintln!("ERROR: cannot perform PSH from register 0x{:04x} to stack", src);
            self.cpu.halt();
        }
        self.trace(&format!("PSH, SRC={}, STACK=0x{:04x}", match src { REG_A => "A", REG_B => "B", REG_C => "C", _ => "?" }, self.cpu.get_sp()));
    }

    fn exec_pop(&mut self, dest: u8) {
        let value = self.cpu.pop().expect("ERROR: [POP] Stack overflow");
        if !self.cpu.write_reg(dest, value) {
            eprintln!("ERROR: cannot perform POP to register 0x{:04x} from stack", dest);
            self.cpu.halt();
        }
        self.trace(&format!("POP, DEST={}, STACK=0x{:04x}", match dest { REG_A => "A", REG_B => "B", REG_C => "C", _ => "?" }, self.cpu.get_sp()));
    }

    fn exec_jof(&mut self, addr: u16) {
        let ok = self.cpu.get_sys_flags() & 0x02 == 0x02;
        if ok { self.cpu.set_pc(addr); }
        self.trace(&format!("JOF, NEXT ADDRESS=0x{:04X} F=0b{:08b} -> {}", addr, self.cpu.get_sys_flags() , if ok { "TAKEN" } else { "NOT TAKEN" }));
    }

    fn exec_xor(&mut self, dest: u8, src: u8) {
        let a = self.cpu.read_reg(dest).expect("ERROR: [XOR] invalid dest register ID");
        let b = self.cpu.read_reg(src).expect("ERROR: [XOR] invalid src register ID");
        let res = a ^ b;
        if !self.cpu.write_reg(dest, res) {
            eprintln!("ERROR: cannot perform XOR to register 0x{:04X}", dest);
            self.cpu.halt();
        }
        self.trace(&format!("XOR, DEST=0x{:04x}, SRC=0x{:04x}, RES=0x{:04x}", a, b, res));
    }

    fn exec_bor(&mut self, dest: u8, src: u8) {
        let a = self.cpu.read_reg(dest).expect("ERROR: [BOR] invalid dest register ID");
        let b = self.cpu.read_reg(src).expect("ERROR: [BOR] invalid src register ID");
        let res = a | b;
        if !self.cpu.write_reg(dest, res) {
            eprintln!("ERROR: cannot perform BOR to register 0x{:04X}", dest);
            self.cpu.halt();
        }
        self.trace(&format!("BOR, DEST=0x{:04x}, SRC=0x{:04x}, RES=0x{:04x}", a, b, res));
    }

    fn exec_and(&mut self, dest: u8, src: u8) {
        let a = self.cpu.read_reg(dest).expect("ERROR: [AND] invalid dest register ID");
        let b = self.cpu.read_reg(src).expect("ERROR: [AND] invalid src register ID");
        let res = a & b;
        if !self.cpu.write_reg(dest, res) {
            eprintln!("ERROR: cannot perform AND to register 0x{:04X}", dest);
            self.cpu.halt();
        }
        self.trace(&format!("AND, DEST=0x{:04x}, SRC=0x{:04x}, RES=0x{:04x}", a, b, res));
    }

    fn exec_mov(&mut self, dest: u8, src: u8) {
        let value = self.cpu.read_reg(src).expect("ERROR: [MOV] invalid register ID");
        if !self.cpu.write_reg(dest, value) {
            eprintln!("ERROR: cannot perform MOV from register 0x{:04X}", src);
            self.cpu.halt();
        }
        self.trace(&format!("MOV, DEST=0x{:04x}, SRC=0x{:04x}, VALUE=0x{:04x}", dest, src, value));
    }

    fn exec_moc(&mut self, dest: u8, val: i8) {
        if !self.cpu.write_reg(dest, val) {
            eprintln!("ERROR: cannot perform MOV with constant 0x{:04X}", val);
            self.cpu.halt();
        }
        self.trace(&format!("MOC, DEST=0x{:04x}, VALUE=0x{:04x}", dest, val));
    }

    fn exec_halt(&mut self) {
        self.trace("HLT");
        self.cpu.halt();
    }


    fn exec_add(&mut self) {
        let a = self.cpu.read_reg(REG_A).expect("ERROR: [ADD] invalid register 'a' ID");
        let b = self.cpu.read_reg(REG_B).expect("ERROR: [ADD] invalid register 'b' ID");
        let (res, over) = a.overflowing_add(b);
        let carry: bool = (a as u16 + b as u16) > 0xFF;
        self.cpu.set_sys_flags(self.calc_sys_flags(res, carry, over));
        if !self.cpu.write_reg(REG_C, res) {
            eprintln!("ERROR: cannot perform ADD");
            self.cpu.halt();
        }
        self.trace(&format!("ADD, A = {}, B = {}, RES = {}, OF = {}, CF = {}", a, b, res, over, carry));
    }

    fn exec_sub(&mut self) {
        let a = self.cpu.read_reg(REG_A).expect("ERROR: [SUB] invalid register 'a' ID");
        let b = self.cpu.read_reg(REG_B).expect("ERROR: [SUB] invalid register 'b' ID");
        let (res, over) = b.overflowing_sub(a);
        let carry: bool = b < a;
        self.cpu.set_sys_flags(self.calc_sys_flags(res, carry, over));
        if !self.cpu.write_reg(REG_C, res) {
            eprintln!("ERROR: cannot perform ADD");
            self.cpu.halt();
        }
        self.trace(&format!("SUB, A = {}, B = {}, RES = {}, OF = {}, CF = {}", a, b, res, over, carry));
    }

    fn exec_mul(&mut self) {
        // TODO: sys flag handling (cf, of)
        let a = self.cpu.read_reg(REG_A).expect("ERROR: [MUL] invalid register 'a' ID");
        let b = self.cpu.read_reg(REG_B).expect("ERROR: [MUL] invalid register 'b' ID");
        let (res, over) = a.overflowing_mul(b);
        if !self.cpu.write_reg(REG_C, res) {
            eprintln!("ERROR: cannot perform ADD");
            self.cpu.halt();
        }
        self.trace(&format!("MUL, A = {}, B = {}, RES = {}, OF = {}", a, b, res, over));
    }

    fn exec_jmp(&mut self, address: u16) {
        self.cpu.set_pc(address);
        self.trace(&format!("JMP, ADDRESS = {}", address));
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
        self.cpu.set_flags(f);
        self.trace(&format!("CMP, A = {}, B = {}, F = {:#b}", a, b, f));
    }

    fn exec_jct(&mut self, mask: u8, address: u16) {
        let flags = self.cpu.get_flags();
        let ok: bool = (flags & mask) == 0;
        if ok { self.cpu.set_pc(address); }
        self.trace(&format!("JCT, mask=0b{:08b}, F=0b{:08b} -> {}", mask, flags, if ok { "TAKEN" } else { "NOT TAKEN" }));
    }

    fn exec_inc(&mut self, address: u8) {
        // TODO: sys flag handling (of, sf)
        let value = self.cpu.read_reg(address).expect("ERROR: [INC] invalid register ID");
        let res = value + 1;
        if !self.cpu.write_reg(address, res) {
            eprintln!("ERROR: cannot perform INC on register {}", address);
            self.cpu.halt();
        }
        self.trace(&format!("INC, REG=0x{:04x}, VAL_PRE={}, VAL_POST={}", address, value, res));
    }

    fn exec_dec(&mut self, address: u8) {
        // TODO: sys flag handling (of, sf)
        let value = self.cpu.read_reg(address).expect("ERROR: [DEC] invalid register ID");
        let res = value - 1;
        if !self.cpu.write_reg(address, res) {
            eprintln!("ERROR: cannot perform DEC on register {}", address);
            self.cpu.halt();
        }
        self.trace(&format!("DEC, REG=0x{:04x}, VAL_PRE={}, VAL_POST={}", address, value, res));
    }

    fn exec_ofs(&mut self, offset: u16) {
        let new_addr = self.cpu.get_pc() + offset;
        self.cpu.set_pc(new_addr);
        self.trace(&format!("OFS, OFFSET={}, NEW={}", offset, new_addr));
    }

    fn exec_jor(&mut self, mask: u8, address: u16) {
        if (mask == 0) || ((mask & (mask - 1)) != 0) {
            self.cpu.halt();
            eprintln!("ERROR: JOR requires exactly one bit in mask (got 0b{:08b})", mask);
            return;
        }
        let flags = self.cpu.get_flags();
        let ok: bool = (flags & mask) != 0;
        if ok { self.cpu.set_pc(address); }
        self.trace(&format!("JOR, mask=0b{:08b}, F=0b{:08b} -> {}", mask, flags, if ok { "TAKEN" } else { "NOT TAKEN" }));
    }

    fn exec_str(&mut self, register_addr: u8, mem_addr: u16) {
        let value = self.cpu.read_reg(register_addr).expect("ERROR: [STR] invalid register ID");
        if !self.cpu.write_mem(mem_addr, value as u8) {
            eprintln!("ERROR: cannot perform STR to address 0x{:04X}", mem_addr);
            self.cpu.halt();
        }
        self.trace(&format!("STR, mem[0x{:04X}] <= {} ({})", mem_addr, match register_addr { REG_A => "A", REG_B => "B", REG_C => "C", _ => "?" }, value));
    }

    fn exec_lmr(&mut self, register_addr: u8, mem_addr: u16) {
        // TODO: should handle sys flags (OF)
        let value = self.cpu.read_mem(mem_addr).expect("ERROR: [LMR] invalid memory address");
        if !self.cpu.write_reg(register_addr, value as i8) {
            eprintln!("ERROR: cannot perform LMR from address 0x{:04X}", mem_addr);
            self.cpu.halt();
        }
        self.trace(&format!("LMR, mem[0x{:04X}] => {} ({})", mem_addr, match register_addr { REG_A => "A", REG_B => "B", REG_C => "C", _ => "?" }, value));
    }

    fn exec_not(&mut self, register_addr: u8) {
        // TODO: sys flag handling (sf)
        let new_val = !(self.cpu.read_reg(register_addr).expect("ERROR: [NOT] invalid register ID"));
        if !self.cpu.write_reg(register_addr, new_val) {
            eprintln!("ERROR: cannot perform NOT for register {}", match register_addr { 0 => "A", 1 => "B", 2 => "C", _ => "?" });
            self.cpu.halt();
        }
        self.trace(&format!("NOT, REG=0x{:04x}, NEW={}", register_addr, new_val));
    }
}
