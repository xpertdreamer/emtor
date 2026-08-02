use crate::reg::*;

pub const MEM_SIZE: usize = 256;
// pub const STACK_SIZE: usize = 16;
// pub const STACK_START: u16 = MEM_SIZE as u16 - STACK_SIZE as u16;

pub struct Cpu {
    regs: Regs,
    pc: u16,
//    sp: u16,
    mem: [u8; MEM_SIZE],
    state: bool
}

impl Cpu {
    pub fn create() -> Self {
        Cpu {
            regs: Regs::create(),
            pc: 0,
            //sp: STACK_START,
            mem: [0; MEM_SIZE],
            state: true
        }
    }

    // pub fn push(&mut self, value: u8) -> bool {}
    // pub fn pop(&mut self) -> Option<u8> {}
    // pub fn push_u16(&mut self, value: u16) -> bool {}
    // pub fn pop_u16(&mut self) -> Option<u16> {}

    pub fn load_prog(&mut self, data: &[u8]) {
        // if data.len() > STACK_START as usize {
        //     eprintln!("WARNING: Program size exceeds stack start at {}", STACK_START);
        // }
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

    pub fn read_mem(&self, address: u16) -> Option<u8> {
        if (address as usize) < MEM_SIZE {
            Some(self.mem[address as usize])
        } else {
            None
        }
    }

    pub fn write_mem(&mut self, address: u16, value: u8) -> bool {
        // if address >= STACK_START || address < STACK_START + STACK_SIZE as u16 {
        //     eprintln!("ERROR: Attempt to write to stack memory at {:#04X}", address);
        //     return false;
        // }
        if (address as usize) < MEM_SIZE {
            self.mem[address as usize] = value;
            true
        } else {
            false
        }
    }

    pub fn read_reg(&self, reg_id: u8) -> Option<u8> {
        match reg_id {
            REG_A => Some(self.regs.a),
            REG_B => Some(self.regs.b),
            REG_C => Some(self.regs.c),
            _ => None,
        }
    }

    pub fn write_reg(&mut self, reg_id: u8, value: u8) -> bool {
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

    pub fn is_running(&self) -> bool {
        self.state
    }

    pub fn halt(&mut self) {
        self.state = false;
    }
}
