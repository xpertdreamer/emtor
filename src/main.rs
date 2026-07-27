mod cpu;
mod reg;

use crate::cpu::Cpu;

fn main() {
    let mut emtor = Cpu::create();
    emtor.regs.a = 5;
    emtor.regs.b = 6;
    emtor.load_prog(&[1, 0]);
    emtor.run();
    println!("{}", emtor.regs.a);
}
