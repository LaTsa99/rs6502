mod cpu;
fn main() {
    let mut cpu : cpu::CPU = cpu::new();
    
    cpu.reset();

    println!("{:#?}", cpu);
    cpu.fetch();
    cpu.execute();
}
