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
        0x00                     // HLT
    ]);

    emtor.run();
    println!("reg a: {}", emtor.regs.a);
    println!("reg b: {}", emtor.regs.b);
    println!("pc: {}", emtor.pc);
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

    #[test]
    fn test_add() {
        let mut emtor = Cpu::create();
        emtor.load_prog(&[
            0x02, 0x0B, 0x00, 12,    // MOV Const
            0x02, 0x0B, 0x01, 2,     // MOV Const
            0x01,                    // ADD
        ]);
        emtor.run();
        assert_eq!(emtor.regs.a, 14);
        assert_eq!(emtor.regs.b, 2);
        assert_eq!(emtor.pc, 10);
    }

    #[test]
    fn test_mul() {
        let mut emtor = Cpu::create();
        emtor.load_prog(&[
            0x02, 0x0B, 0x00, 0x32,     // MOV Const
            0x02, 0x0B, 0x01, 0x02,     // MOV Const
            0x04                        // MUL
        ]);
        emtor.run();
        assert_eq!(emtor.regs.a, 0x64);
        assert_eq!(emtor.regs.b, 0x02);
        assert_eq!(emtor.pc, 0x0A);
    }

    #[test]
    fn test_jmp() {
        let mut emtor = Cpu::create();
        emtor.load_prog(&[
            0x02, 0x0B, 0x00, 0x32,     // MOV Const    1
            0x02, 0x0B, 0x01, 0x02,     // MOV Const    2
            0x05, 0x00, 0x10,           // JMP -> 5     3
            0x02, 0x0A, 0x00, 0x01,     // MOV Reg      4
            0x00                        // HLT          5
        ]);
        emtor.run();
        assert_eq!(emtor.regs.a, 0x32);
        assert_eq!(emtor.regs.b, 0x02);
        assert_eq!(emtor.pc, 0x11);
    }
}
