pub const REG_A: u8 = 0;
pub const REG_B: u8 = 1;

pub struct Regs {
    pub a: u8,
    pub b: u8
}

impl Regs {
    pub fn create() -> Self {
        Regs { a: 0, b: 0 }
    }
}
