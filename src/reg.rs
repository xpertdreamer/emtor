pub struct Regs {
    pub a: u8,
    pub b: u8
}

impl Regs {
    pub fn create() -> Self {
        Regs { a: 0, b: 0 }
    }
}
