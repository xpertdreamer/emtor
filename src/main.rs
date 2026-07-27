mod cpu;
mod reg;

use crate::cpu::{Cpu, MEM_SIZE};

fn main() {
    let mut emtor = Cpu::create();
    emtor.regs.a = 5;
    emtor.regs.b = 6;
    emtor.load_prog(&[1, 1, 1, 0]);
    emtor.run();
    println!("reg a: {}", emtor.regs.a);
    println!("reg b: {}", emtor.regs.b);
    println!("pc: {}", emtor.pc);
    println!("----------");
    for i in 0..MEM_SIZE {
        if emtor.mem[i] != 0 {
            println!("{} = {}", i, emtor.mem[i]);
        }
    }
}
