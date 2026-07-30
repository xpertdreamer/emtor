mod cpu;
mod reg;

pub const TRACE: bool = true;

use crate::cpu::{Cpu, MEM_SIZE};

fn main() {
    let mut emtor = Cpu::create();

    emtor.load_prog(&[
        0x02, 0x0B, 0x00, 12,    // MOV Const
        0x02, 0x0B, 0x01, 2,     // MOV Const
        0x01,                    // ADD
        0x02, 0x0A, 0x01, 0x00,  // MOV Reg
        0x02, 0x0B, 0x01, 0x04,  // MOV Const
        0x03,                    // SUB
        0x04,                    // MUL
        0x06,                    // CMP
        0x00                     // HLT
    ]);

    emtor.run();
    println!("reg a: {}", emtor.regs.a);
    println!("reg b: {}", emtor.regs.b);
    println!("pc: {}", emtor.pc);
    println!("reg f {:#b}", emtor.regs.f);
    println!("----------");
    for i in 0..MEM_SIZE {
        if emtor.mem[i] != 0 {
            println!("{} = {:#X}", i, emtor.mem[i]);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn run_test(program: &[u8]) -> Cpu {
        let mut cpu = Cpu::create();
        cpu.load_prog(program);
        cpu.run();
        cpu
    }

    #[test]
    fn test_add() {
        let emtor = run_test(&[
            0x02, 0x0B, 0x00, 12,    // MOV Const
            0x02, 0x0B, 0x01, 2,     // MOV Const
            0x01,                    // ADD
        ]);
        assert_eq!(emtor.regs.c, 14);
        assert_eq!(emtor.regs.b, 2);
        assert_eq!(emtor.pc, 10);
    }

    #[test]
    fn test_mul() {
        let emtor = run_test(&[
            0x02, 0x0B, 0x00, 0x32,     // MOV Const
            0x02, 0x0B, 0x01, 0x02,     // MOV Const
            0x04                        // MUL
        ]);
        assert_eq!(emtor.regs.c, 0x64);
        assert_eq!(emtor.regs.b, 0x02);
        assert_eq!(emtor.pc, 0x0A);
    }

    #[test]
    fn test_jmp() {
        let emtor = run_test(&[
            0x02, 0x0B, 0x00, 0x32,     // MOV Const    0
            0x02, 0x0B, 0x01, 0x02,     // MOV Const    1
            0x05, 0x00, 0x10,           // JMP -> 5     2
            0x02, 0x0A, 0x00, 0x01,     // MOV Reg      3
            0x00                        // HLT          4
        ]);
        assert_eq!(emtor.regs.a, 0x32);
        assert_eq!(emtor.regs.b, 0x02);
        assert_eq!(emtor.pc, 0x11);
    }

    #[test]
    fn test_jct() {
        let emtor = run_test(&[
            0x02, 0x0B, 0x00, 0x03,     // MOV Const    0
            0x08, 0x01,                 // INC B        1
            0x06,                       // CMP          2
            0x07, 0x96, 0x00, 0x04,     // JCT          3
        ]);
        assert_eq!(emtor.regs.a, 0x03);
        assert_eq!(emtor.regs.b, 0x03);
        assert_eq!(emtor.pc, 0xC);
        assert_eq!(emtor.regs.f, 0xB1);
    }
}
