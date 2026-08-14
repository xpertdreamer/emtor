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
    fn add() {
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
    fn div() {
        let emtor = run_test(&[
            0x0F, 0xC0, 0x06,   // MOC A
            0x0F, 0xC1, 0x0C,   // MOC B
            0x20,               // DIV
            0x00                // HLT
        ]);
        let dump = emtor.dump();
        assert_eq!(dump.reg_a, 0x06);
        assert_eq!(dump.reg_b, 0x0C);
        assert_eq!(dump.reg_c, 0x02);
    }

    #[test]
    fn div_of() {
        let emtor = run_test(&[
            0x0F, 0xC0, 0x00,   // MOC A
            0x0F, 0xC1, 0x06,   // MOC B
            0x20,               // DIV
            0x14, 0x00, 0x0B,   // JOF
            0x00,               // HLT
            0x08, 0xC0,         // INC A
            0x00,               // HLT
        ]);
        let dump = emtor.dump();
        assert_eq!(dump.reg_a, 0x01);
        assert_eq!(dump.reg_b, 0x06);
        assert_eq!(dump.reg_c, 0x00);
        assert_eq!(dump.pc, 0x0E);
    }

    #[test]
    fn mov() {
        let emtor = run_test(&[
            0x0F, 0xC0, 12,     // MOC
            0x02, 0xC1, 0xC0,   // MOV
        ]);
        let dump = emtor.dump();
        assert_eq!(dump.reg_a, 12);
        assert_eq!(dump.reg_b, 12);
    }

    #[test]
    fn sub() {
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
    fn mul() {
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
    fn jmp() {
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
    fn cmp() {
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
    fn jct() {
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
    fn dec() {
        let emtor = run_test(&[
            0x0F, 0xC0, 0x07,   // MOC
            0x09, 0xC0,         // DEC
            0x00                // HLT
        ]);
        let dump = emtor.dump();
        assert_eq!(dump.reg_a, 0x06);
    }

    #[test]
    fn ofs() {
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
    fn nop() {
        let cpu = run_test(&[
            0x0C, 0x0C, 0x0C,   // NOP 3T
            0x0C, 0x0C          // NOP 2T
        ]);
        let dump = cpu.dump();
        assert_eq!(dump.pc, 0x06);
    }

    #[test]
    fn str() {
        let cpu = run_test(&[
            0x0F, 0xC0, 0x01,           // MOC
            0x0D, 0xC0, 0x00, 0x96,     // STR
            0x00                        // HLT
        ]);
        let dump = cpu.dump();
        assert_eq!(dump.pc, 8);
        assert_eq!(dump.reg_a, 1);
        assert_eq!(dump.mem[0x0096], 0x01);
    }

    #[test]
    fn lmr() {
        let mut prog: [u8; 201] = [0; 201];
        prog[..5].copy_from_slice(&[
            0x0E, 0xC0, 0x00, 0xC8,     // LMR
            0x00                        // HLT
        ]);
        prog[0xC8] = 0x63;
        let cpu = run_test(&prog);
        let dump = cpu.dump();
        assert_eq!(dump.mem[0x00C8], 0x63);
        assert_eq!(dump.pc, 5);
        assert_eq!(dump.reg_a, 0x63);
    }

    #[test]
    fn not() {
        let cpu = run_test(&[
            0x0F, 0xC0, 0x28,   // MOC
            0x10, 0xC0,         // NOT
            0x00                // HLT
        ]);
        let dump = cpu.dump();
        assert_eq!(dump.pc, 6);
        assert_eq!(dump.reg_a, 0xD7u8 as i8);
    }

    #[test]
    fn xor() {
        let cpu = run_test(&[
            0x0F, 0xC0, 0x84,   // MOC A
            0x0F, 0xC1, 0x96,   // MOC B
            0x11, 0xC0, 0xC1,   // XOR A B
            0x00                // HLT
        ]);
        let dump = cpu.dump();
        assert_eq!(dump.pc, 10);
        assert_eq!(dump.reg_a, 0x12);
        assert_eq!(dump.reg_b, 0x96u8 as i8);
    }

    #[test]
    fn bor() {
        let cpu = run_test(&[
            0x0F, 0xC0, 0x67,   // MOC A
            0x0F, 0xC1, 0x76,   // MOC B
            0x12, 0xC1, 0xC0,   // BOR B A
            0x00                // HLT
        ]);
        let dump = cpu.dump();
        assert_eq!(dump.pc, 10);
        assert_eq!(dump.reg_a, 0x67);
        assert_eq!(dump.reg_b, 0x77);
    }

    #[test]
    fn and() {
        let cpu = run_test(&[
            0x0F, 0xC0, 0x23,   // MOC A
            0x0F, 0xC1, 0x32,   // MOC B
            0x13, 0xC1, 0xC0,   // BOR B A
            0x00                // HLT
        ]);
        let dump = cpu.dump();
        assert_eq!(dump.pc, 10);
        assert_eq!(dump.reg_a, 0x23);
        assert_eq!(dump.reg_b, 0x22);
    }

    #[test]
    fn jof() {
        let cpu = run_test(&[
            0x0F, 0xC0, 0x7F,   // MOC A
            0x02, 0xC1, 0xC0,   // MOV A->B
            0x01,               // ADD
            0x14, 0x00, 0x0B,   // JOF
            0x00,               // HLT
            0x08, 0xC0,         // INC A
            0x00                // HLT
        ]);
        let dump = cpu.dump();
        assert_eq!(dump.pc, 14);
        assert_eq!(dump.reg_a, 0x80u8 as i8);
        assert_eq!(dump.reg_b, 0x7Fu8 as i8);
        assert_eq!(dump.reg_c, 0xFEu8 as i8);
    }

    #[test]
    fn psh() {
        let cpu = run_test(&[
            0x0F, 0xC0, 0x71,   // MOC A
            0x15, 0xC0,         // PUSH A
            0x00                // HLT
        ]);
        let dump = cpu.dump();
        assert_eq!(dump.pc, 6);
        assert_eq!(dump.reg_a, 0x71);
        assert_eq!(dump.sp, 0xf1);
        assert_eq!(dump.mem[0x00f0], 0x71);
    }

    #[test]
    fn pop() {
        let cpu = run_test(&[
            0x0F, 0xC0, 0x71,   // MOC A
            0x15, 0xC0,         // PUSH A
            0x16, 0xC1,         // POP B
            0x00                // HLT
        ]);
        let dump = cpu.dump();
        assert_eq!(dump.pc, 8);
        assert_eq!(dump.reg_a, 0x71);
        assert_eq!(dump.sp, 0xf0);
        assert_eq!(dump.mem[0x00f0], 0x71);
        assert_eq!(dump.reg_b, 0x71);
    }

    #[test]
    fn push_stack_overflow() {
        let cpu = run_test(&[
            0x0F, 0xC0, 0x71,   // MOC A
            0x15, 0xC0,         // PUSH A
            0x15, 0xC0,         // PUSH A
            0x15, 0xC0,         // PUSH A
            0x15, 0xC0,         // PUSH A
            0x15, 0xC0,         // PUSH A
            0x15, 0xC0,         // PUSH A
            0x15, 0xC0,         // PUSH A
            0x15, 0xC0,         // PUSH A
            0x15, 0xC0,         // PUSH A
            0x15, 0xC0,         // PUSH A
            0x15, 0xC0,         // PUSH A
            0x15, 0xC0,         // PUSH A
            0x15, 0xC0,         // PUSH A
            0x15, 0xC0,         // PUSH A
            0x15, 0xC0,         // PUSH A
            0x15, 0xC0,         // PUSH A
            0x15, 0xC0,         // PUSH A
            0x15, 0xC0,         // PUSH A
            0x15, 0xC0,         // PUSH A
            0x15, 0xC0,         // PUSH A
            0x15, 0xC0,         // PUSH A
            0x15, 0xC0,         // PUSH A
            0x15, 0xC0,         // PUSH A
            0x15, 0xC0,         // PUSH A
            0x15, 0xC0,         // PUSH A
            0x15, 0xC0,         // PUSH A
            0x15, 0xC0,         // PUSH A
            0x15, 0xC0,         // PUSH A
            0x15, 0xC0,         // PUSH A
            0x15, 0xC0,         // PUSH A
            0x15, 0xC0,         // PUSH A
            0x15, 0xC0,         // PUSH A
        ]);
        let dump = cpu.dump();
        assert_eq!(dump.reg_a, 0x71);
        assert_eq!(dump.sp, 0x100);
        assert_eq!(dump.mem[0x00f0], 0x71);
    }

    #[test]
    #[should_panic(expected="ERROR: [POP] Stack overflow")]
    fn pop_stack_overflow() {
        let _ = run_test(&[
            0x0F, 0xC1, 0x01,   // MOC B
            0x16, 0xC1,         // POP B
            0x00,               // HLT
        ]);
    }

    #[test]
    #[should_panic(expected="ERROR: [PSH] invalid src register ID")]
    fn invalid_register_address() {
        let _ = run_test(&[
            0x15, 0xC8, 0x01,   // PSH ?
            0x00,               // HLT
        ]);
    }

    #[test]
    fn modulo() {
        let cpu = run_test(&[
            0x0F, 0xC0, 0x51,   // MOC A
            0x0F, 0xC1, 0x75,   // MOC B
            0x17,               // MOD
            0x00                // HLT
        ]);
        let dump = cpu.dump();
        assert_eq!(dump.pc, 8);
        assert_eq!(dump.reg_a, 0x51);
        assert_eq!(dump.reg_b, 0x75);
        assert_eq!(dump.reg_c, 0x51);
    }

    #[test]
    fn iwg() {
        let cpu = run_test(&[
            0x0F, 0xC0, 0x01,   // MOC A
            0x0F, 0xC1, 0x05,   // MOC B
            0x18, 0x00, 0x0A,   // IWG
            0x00,               // HLT
            0x08, 0xC0,         // INC A
            0x08, 0xC1,         // INC B
            0x00                // HLT
        ]);
        let dump = cpu.dump();
        assert_eq!(dump.pc, 15);
        assert_eq!(dump.reg_a, 0x02);
        assert_eq!(dump.reg_b, 0x06);
        assert_eq!(dump.sp, 0xF0);
        assert_eq!(dump.csp, 0xE2);
        assert_eq!(dump.mem[(dump.csp - 0x01) as usize], 0x09);
        assert_eq!(dump.mem[(dump.csp - 0x02) as usize], 0x00);
    }

    #[test]
    fn gmb() {
        let cpu = run_test(&[
            0x0F, 0xC0, 0x01,   // MOC A
            0x0F, 0xC1, 0x05,   // MOC B
            0x18, 0x00, 0x0A,   // IWG
            0x00,               // HLT
            0x08, 0xC0,         // INC A
            0x08, 0xC1,         // INC B
            0x19,               // GMB
            0x00                // HLT
        ]);
        let dump = cpu.dump();
        assert_eq!(dump.pc, 10);
        assert_eq!(dump.reg_a, 0x02);
        assert_eq!(dump.reg_b, 0x06);
        assert_eq!(dump.sp, 0xF0);
        assert_eq!(dump.csp, 0xE0);
    }

    #[test]
    fn multiple_iwg_gmb() {
        let cpu = run_test(&[
            0x0F, 0xC0, 0x01,   // MOC A
            0x0F, 0xC1, 0x05,   // MOC B
            0x18, 0x00, 0x0A,   // IWG
            0x00,               // HLT
            0x08, 0xC0,         // INC A
            0x08, 0xC1,         // INC B
            0x18, 0x00, 0x13,   // IWG
            0x19,               // GMB
            0x00,               // HLT
            0x09, 0xC1,         // DEC B
            0x04,               // MUL
            0x09, 0xC0,         // DEC A
            0x19,               // GMB
            0x00                // HLT
        ]);
        let dump = cpu.dump();
        assert_eq!(dump.pc, 10);
        assert_eq!(dump.reg_a, 0x01);
        assert_eq!(dump.reg_b, 0x05);
        assert_eq!(dump.reg_c, 0x0A);
        assert_eq!(dump.sp, 0xF0);
        assert_eq!(dump.csp, 0xE0);
    }


    // 1000 0001 << 1000 0100
    #[test]
    fn sht() {
        let cpu = run_test(&[
            0x0F, 0xC0, 0x81,   // MOC A
            0x21, 0x02, 0xC0,   // SHT A
            0x00,               // HLT
        ]);
        let dump = cpu.dump();
        assert_eq!(dump.pc, 7);
        assert_eq!(dump.reg_a, 0x84u8 as i8);
    }

    // 0000 0001 << 0000 0101
    #[test]
    fn shc() {
        let cpu = run_test(&[
            0x0F, 0xC0, 0x01,   // MOC A
            0x0F, 0xC1, 0xFF,   // MOC B
            0x01,               // ADD
            0x22, 0x02, 0xC0,   // SHC A
            0x00,               // HLT
        ]);
        let dump = cpu.dump();
        assert_eq!(dump.pc, 11);
        assert_eq!(dump.reg_a, 0x05);
    }

    // 0000 0001 >> 0100 0000
    #[test]
    fn rtr() {
        let cpu = run_test(&[
            0x0F, 0xC0, 0x01,   // MOC A
            0x23, 0x82, 0xC0,   // RTR A
            0x00,               // HLT
        ]);
        let dump = cpu.dump();
        assert_eq!(dump.pc, 7);
        assert_eq!(dump.reg_a, 0x40u8 as i8);
    }

    // 0000 0001 << 0000 0100
    #[test]
    fn bsl() {
        let cpu = run_test(&[
            0x0F, 0xC0, 0x01,   // MOC A
            0x24, 0x02, 0xC0,   // BSL A <<
            0x00,               // HLT
        ]);
        let dump = cpu.dump();
        assert_eq!(dump.pc, 7);
        assert_eq!(dump.reg_a, 0x04);
    }

    // TODO: complicated tests
}
