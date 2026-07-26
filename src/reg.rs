pub struct Regs {
    a: u8,
    b: u8
}

impl Regs {
    pub fn create() -> Self {
        Regs { a: 0, b: 0 }
    }
}
