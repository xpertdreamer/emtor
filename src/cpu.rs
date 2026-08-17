use crate::reg::*;

pub const MEM_SIZE: usize = 256;
pub const STACK_SIZE: usize = MEM_SIZE / 16;
pub const STACK_START: u16 = MEM_SIZE as u16 - STACK_SIZE as u16;
pub const CALL_STACK_SIZE: usize = MEM_SIZE / 16;
pub const CALL_STACK_START: u16 = STACK_START  - CALL_STACK_SIZE as u16;
pub const DATA_SEG_SIZE: usize = (MEM_SIZE - STACK_SIZE - CALL_STACK_SIZE) / 2;
pub const DATA_SEG_START: u16 = (MEM_SIZE - DATA_SEG_SIZE) as u16;

pub struct Cpu {
    regs: Regs,
    pc: u16,
    sp: u16,
    csp: u16,
    mem: [u8; MEM_SIZE],
    state: bool
}

impl Cpu {
    pub fn create() -> Self {
        Cpu {
            regs: Regs::create(),
            pc: 0,
            sp: STACK_START,
            csp: CALL_STACK_START,
            mem: [0; MEM_SIZE],
            state: true
        }
    }

    pub fn push(&mut self, value: i8) -> bool {
        if self.sp == MEM_SIZE as u16 {
            eprintln!("ERROR: Stack overflow - sp={}, max={}", self.sp, STACK_START + STACK_SIZE as u16 - 1);
            return false;
        }
        self.mem[self.sp as usize] = value as u8;
        self.sp += 1;
        true
    }

    pub fn pop(&mut self) -> Option<i8> {
        if self.sp == STACK_START {
            eprintln!("ERROR: Stack overflow - sp={}, min={}", self.sp, STACK_START);
            return None;
        }
        self.sp -= 1;
        Some(self.mem[self.sp as usize] as i8)
    }

    pub fn push_call(&mut self, value: u16) -> bool {
        if self.csp == STACK_START - 2 {
            eprintln!("ERROR: Call stack overflow - csp={}, max={}", self.csp, CALL_STACK_START + CALL_STACK_SIZE as u16 - 1);
            return false;
        }

        self.mem[self.csp as usize] = (value >> 8) as u8;
        self.mem[(self.csp + 1) as usize] = (value & 0xFF) as u8;
        self.csp += 2;
        true
    }

    pub fn pop_call(&mut self) -> Option<u16> {
        if self.csp < CALL_STACK_START + 2 {
            eprintln!("ERROR: Call stack overflow - csp={}, min={}", self.csp, CALL_STACK_START);
            return None;
        }
        self.csp -= 2;
        Some((self.mem[self.csp as usize] as u16) << 8 | (self.mem[(self.csp + 1) as usize] as u16))
    }

    pub fn load_prog(&mut self, data: &[u8]) {
        if data.len() > DATA_SEG_START as usize {
            eprintln!("WARNING: Program size exceeds data segment at {}", DATA_SEG_START);
        }
        for (i, &byte) in data.iter().enumerate() {
            self.mem[i] = byte;
        }
        self.pc = 0;
        self.regs.f_zeroed();
    }

    pub fn fetch_next_byte(&mut self) -> Option<u8> {
        if (self.pc as usize) < MEM_SIZE {
            let byte = self.mem[self.pc as usize];
            self.pc += 1;
            Some(byte)
        } else {
            self.state = false;
            eprintln!("ERROR: End of memory");
            None
        }
    }

    pub fn get_mem(&self) -> [u8; MEM_SIZE] {
        self.mem
    }

    pub fn read_mem(&self, address: u16) -> Option<u8> {
        if (address as usize) < MEM_SIZE {
            Some(self.mem[address as usize])
        } else {
            None
        }
    }

    pub fn write_mem(&mut self, address: u16, value: u8) -> bool {
        let addr = DATA_SEG_START + address;
        if addr >= CALL_STACK_START {
            eprintln!("ERROR: Attempt to write to stack memory at {:#04X}", addr);
            return false;
        }
        if (addr as usize) < MEM_SIZE {
            self.mem[addr as usize] = value;
            true
        } else {
            eprintln!("ERROR: Memory out of bounds at {:#04X}", addr);
            false
        }
    }

    pub fn read_reg(&self, reg_id: u8) -> Option<i8> {
        match reg_id {
            REG_A => Some(self.regs.a),
            REG_B => Some(self.regs.b),
            REG_C => Some(self.regs.c),
            _ => None,
        }
    }

    pub fn write_reg(&mut self, reg_id: u8, value: i8) -> bool {
        match reg_id {
            REG_A => self.regs.a = value,
            REG_B => self.regs.b = value,
            REG_C => self.regs.c = value,
            _ => return false,
        }
        true
    }

    pub fn set_flags(&mut self, flags: u8) {
        self.regs.f = flags;
    }

    pub fn get_flags(&self) -> u8 {
        self.regs.f
    }

    pub fn set_sys_flags(&mut self, sflags: u8) {
        self.regs.sf = sflags;
    }

    pub fn get_sys_flags(&self) -> u8 {
        self.regs.sf
    }

    pub fn set_pc(&mut self, address: u16) {
        self.pc = address;
    }

    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    pub fn get_sp(&self) -> u16 {
        self.sp
    }

    pub fn get_csp(&self) -> u16 {
        self.csp
    }

    pub fn is_running(&self) -> bool {
        self.state
    }

    pub fn halt(&mut self) {
        self.state = false;
    }
}
