use std::fmt;

struct CPU {
    // program counter
    pc : u16,

    // stack pointer
    sp : u16,

    // accumulator register
    a : u8,

    // index register
    x : u8,
    y : u8,

    // flags
    n : u8, // negative
    v : u8, // overflow
    b : u8, // break
    d : u8, // decimal
    i : u8, // interrupt disable
    z : u8, // zero flag
    c : u8, // carry flag
}

impl fmt::Debug for CPU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CPU")
            .field("Program Counter", &self.pc)
            .field("Stack Pointer", &self.sp)
            .field("Register A", &self.a)
            .field("Register X", &self.x)
            .field("Register Y", &self.y)
            .field("Negative", &self.n)
            .field("Overflow", &self.v)
            .field("Break", &self.b)
            .field("Decimal", &self.d)
            .field("Interrupt", &self.i)
            .field("Zero Flag", &self.z)
            .field("Carry Flag", &self.c)
            .finish()
    }
}

impl CPU {
    fn fetch(&self){
        println!("Fetch");
    }

    fn execute(&self){
        println!("Execution");
    }
}

fn main() {
    let cpu : CPU = CPU{
        pc : 0, sp : 0, a : 0, x : 0, y : 0,
        n : 0, v : 0, b : 0, d : 0, i : 0, z : 0, c : 0,
    };
    
    println!("{:?}", cpu);
    cpu.fetch();
    cpu.execute();
}
