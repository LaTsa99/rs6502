use std::fmt;

pub struct CPU {
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

    // onboard memory
    mem: [u8 ; rs6502::MAX_MEM],
}

impl fmt::Debug for CPU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CPU")
            .field("Program Counter", &format_args!("0x{0:X}", &self.pc))
            .field("Stack Pointer", &format_args!("0x{0:X}", &self.sp))
            .field("Register A", &format_args!("0x{0:X}", &self.a))
            .field("Register X", &format_args!("0x{0:X}", &self.x))
            .field("Register Y", &format_args!("0x{0:X}", &self.y))
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
    pub fn fetch(&self){
        println!("Fetch");
    }

    pub fn execute(&self){
        println!("Execution");
    }

    pub fn reset(&mut self){
        // set Interrupt disable flag
        self.i = 1;

        // set RESET vector address into PC
        self.pc = rs6502::RESET_VEC;
    }
}

pub fn new() -> CPU {
    CPU{
        pc : 0, sp : 0, a : 0, x : 0, y : 0,
        n : 0, v : 0, b : 0, d : 0, i : 0, z : 0, c : 0,
        mem: [0 ; rs6502::MAX_MEM],
    }
}