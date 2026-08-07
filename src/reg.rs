pub const REG_A: u8 = 0xC0;
pub const REG_B: u8 = 0xC1;
pub const REG_C: u8 = 0xC2;

pub const AR_EMPTY: u16 = 0xFFFF;

pub struct Regs {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub ar: u16,
    pub f: u8,
    pub sf: u8
}

impl Regs {
    pub fn create() -> Self {
        Regs { a: 0, b: 0, c: 0, ar: AR_EMPTY, f: 0, sf: 0 }
    }

    pub fn f_zeroed(&mut self) {
        self.f = 0;
        self.sf = 0;
        self.ar = AR_EMPTY;
    }
}
