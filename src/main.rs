mod cpu;
mod reg;

pub const TRACE: bool = true;

use crate::cpu::{Cpu, MEM_SIZE};

fn main() {
    let mut emtor = Cpu::create();

    emtor.load_prog(&[
        0x02, 0x0B, 0x00, 12,    // MOV Const
        0x02, 0x0B, 0x01, 2,     // MOV Const
        0x01,                    // ADD: A = A + B (12 + 2 = 14)
        0x02, 0x0A, 0x01, 0x00,  // MOV Reg (0x0A)
        0x02, 0x0B, 0x01, 0x04,  // MOV Const
        0x03,                    // SUB
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
