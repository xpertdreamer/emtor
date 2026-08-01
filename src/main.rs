mod cpu;
mod reg;
mod opcode;
mod machine;

pub const TRACE: bool = true;

fn main() {
    // Thats your sandbox
}

#[cfg(test)]
mod tests {
    use crate::machine::Machine;

    fn run_test(program: &[u8]) -> Machine {
        let mut cpu = Machine::create();
        cpu.load_program(program);
        cpu.run();
        cpu
    }

    #[test]
    fn test_add() {
        let emtor = run_test(&[
            0x0F, 0xC0, 12,    // MOC
            0x0F, 0xC1, 2,     // MOC
            0x01,              // ADD
        ]);
        let dump = emtor.dump();
        assert_eq!(dump.reg_a, 12);
        assert_eq!(dump.reg_b, 2);
        assert_eq!(dump.reg_c, 14);
        assert_eq!(dump.pc, 8);
    }

    #[test]
    fn test_mov() {
        let emtor = run_test(&[
            0x0F, 0xC0, 12,     // MOC
            0x02, 0xC1, 0xC0,   // MOV
        ]);
        let dump = emtor.dump();
        assert_eq!(dump.reg_a, 12);
        assert_eq!(dump.reg_b, 12);
    }

    #[test]
    fn test_sub() {
        let emtor = run_test(&[
            0x0F, 0xC0, 2,     // MOC
            0x0F, 0xC1, 12,    // MOC
            0x03,              // ADD
        ]);
        let dump = emtor.dump();
        assert_eq!(dump.reg_a, 2);
        assert_eq!(dump.reg_b, 12);
        assert_eq!(dump.reg_c, 10);
        assert_eq!(dump.pc, 8);
    }

    #[test]
    fn test_mul() {
        let emtor = run_test(&[
            0x0F, 0xC0, 0x32,     // MOC
            0x0F, 0xC1, 0x02,     // MOC
            0x04                  // MUL
        ]);
        let dump = emtor.dump();
        assert_eq!(dump.reg_c, 0x64);
        assert_eq!(dump.reg_b, 0x02);
        assert_eq!(dump.pc, 0x08);
    }

    #[test]
    fn test_jmp() {
        let emtor = run_test(&[
            0x0F, 0xC0, 0x01,     // MOC
            0x05, 0x00, 0x09,     // JMP -> HLT
            0x0F, 0xC0, 0x02,     // MOC
            0x00                  // HLT
        ]);
        let dump = emtor.dump();
        assert_eq!(dump.reg_a, 0x01);
        assert_eq!(dump.pc, 0x0A);
    }

    #[test]
    fn test_cmp() {
        let emtor = run_test(&[
            0x0F, 0xC0, 0x01,     // MOC
            0x0F, 0xC1, 0x02,     // MOC
            0x06,                 // CMP
            0x00                  // HLT
        ]);
        let dump = emtor.dump();
        assert_eq!(dump.reg_a, 0x01);
        assert_eq!(dump.reg_b, 0x02);
        assert_eq!(dump.flags, 0xAA);
    }

    #[test]
    fn test_jct() {
        let emtor = run_test(&[
            0x0F, 0xC0, 0x01,     // MOC
            0x0F, 0xC1, 0x02,     // MOC
            0x06,                 // CMP
            0x07, 0xAA, 0x0B,     // JCT -> INC B
            0x00,                 // HLT
            0x08, 0xC1,           // INC B
            0x00                  // HLT
        ]);
        let dump = emtor.dump();
        assert_eq!(dump.reg_a, 0x01);
        assert_eq!(dump.reg_b, 0x03);
        assert_eq!(dump.flags, 0xAA);
        assert_eq!(dump.pc, 14);
    }

    #[test]
    fn test_dec() {
        let emtor = run_test(&[
            0x0F, 0xC0, 0x07,   // MOC
            0x09, 0xC0,         // DEC
            0x00                // HLT
        ]);
        let dump = emtor.dump();
        assert_eq!(dump.reg_a, 0x06);
    }

    #[test]
    fn test_ofs() {
        let cpu = run_test(&[
            0x0A, 0x00, 0x01,   // 0 OFS -> 0
            0x00,               // 1 HLT
            0x08, 0xC0          // 2 INC
        ]);
        let dump = cpu.dump();
        assert_eq!(dump.reg_a, 0x01);
        assert_eq!(dump.pc, 0x07);
    }

    #[test]
    fn test_nop() {
        let cpu = run_test(&[
            0x0C, 0x0C, 0x0C,   // NOP 3T
            0x0C, 0x0C          // NOP 2T
        ]);
        let dump = cpu.dump();
        assert_eq!(dump.pc, 0x06);
    }

    // TODO: simple tests for other instructions
    // TODO: complicated tests
}
