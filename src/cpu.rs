use std::fmt;

#[allow(dead_code)]
#[derive(Copy, Clone)]
enum AddressingMode {
    // from https://csh.rit.edu/~moffitt/docs/6502.html
    AddrModeUndefined,      
    AddrModeABS,        // Absolute
    AddrModeABSX,       // Absolute Indexed with X
    AddrModeABSY,       // Absolute Indexed with Y
    AddrModeA,          // Accumulator
    AddrModeImmed,      // Immediate
    AddrModeImplied,    // Implied
    AddrModeIndirect,   // Indirect
    AddrModeIndX,       // Indexed Indirect with X
    AddrModeIndY,       // Indexed Indirect with Y
    AddrModeRelative,   // Relative
    AddrModeZP,         // Zero-Page
    AddrModeZPX,        // Zero-Page Indexed with X
    AddrModeZPY,        // Zero-Page Indexed with Y
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
enum InstructionMnemonic {
    // from https://csh.rit.edu/~moffitt/docs/6502.html
    InstrUndefined,
    InstrADC,       // Add Memory to A with Carry
    InstrAND,       // Bitwise AND A with Memory
    InstrASL,       // Arithmetic Shift Left
    InstrBCC,       // Branch if Carry Flag is CLEAR
    InstrBCS,       // Branch if Carry Flag is SET
    InstrBEQ,       // Branch if Zero Flag is SET
    InstrBIT,       // Test bits in A with Memory
    InstrBMI,       // Branch if Sign Flag is SET
    InstrBNE,       // Branch if Zero Flag is CLEAR
    InstrBPL,       // Branch if Sign Flag is CLEAR
    InstrBRK,       // Simulate Interrup Reques (IRQ) == break
    InstrBVC,       // Branch if Overflow Flag is CLEAR
    InstrBVS,       // Branch if Overflow Flag is SET
    InstrCLC,       // Clear Carry Flag
    InstrCLD,       // Clear Binary Coded Decimal Flag
    InstrCLI,       // Clear Interrup Disable Flag
    InstrCLV,       // Clear Overflow Flag
    InstrCMP,       // Compare A with Memory
    InstrCPX,       // Compare X with Memory
    InstrCPY,       // Compare Y with Memory
    InstrDEC,       // Decrement Memory by 1
    InstrDEX,       // Decrement X by 1
    InstrDEY,       // Decrement Y by 1
    InstrEOR,       // Bitwise XOR A with Memory
    InstrInc,       // Increase Memory by 1
    InstrINX,       // Increase X by 1
    InstrINY,       // Increase Y by 1
    InstrJMP,       // GOTO Address
    InstrJSR,       // Jump to Subroutine
    InstrLDA,       // Load A with Memory
    InstrLDX,       // Load X with Memory
    InstrLDY,       // Load Y with Memory
    InstrLSR,       // Logical Shift Right
    InstrNOP,       // No Operation
    InstrORA,       // Bitwise OR A with Memory
    InstrPHA,       // Push A onto Stack
    InstrPHP,       // Push P onto Stack (flags)
    InstrPLA,       // Pull from Stack to A
    InstrPLP,       // Pull from Stack to P (flags)
    InstrROL,       // Rotate Left
    InstrROR,       // Rotate Right
    InstrRTI,       // Return from Interrupt
    InstrRTS,       // Return from Subroutine
    InstrSBC,       // Subtract Memory from A with Borrow
    InstrSEC,       // Set Carry Flag
    InstrSED,       // Set Binary Coded Decimal Flag
    InstrSEI,       // Set Interrupt Disable Flag
    InstrSTA,       // Store A in Memory
    InstrSTX,       // Store X in Memory
    InstrSTY,       // Store Y in Memory
    InstrTAX,       // Transfer A to X
    InstrTAY,       // Transfer A to Y
    InstrTSX,       // Transfer SP to X
    InstrTXA,       // Transfer X to A
    InstrTXS,       // Transfer X to SP
    InstrTYA,       // Transfer Y to A
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
struct Instruction {
    mnem : InstructionMnemonic,     // Mnemonic of Instruction
    length : u8,                    // Length of Instruction
    cycles : u8,                    // Number of Cycles used by Instruction
    mode : AddressingMode,          // Addressing Mode of Instruction
}

#[allow(dead_code)]
pub struct CPU {
    // Program Counter
    pc : u16,

    // Stack Pointer
    sp : u16,

    // Accumulator Register
    a : u8,

    // Index Register
    x : u8,
    y : u8,

    // Flags
    n : u8, // Sign Flag
    v : u8, // Overflow Flag
    b : u8, // Break Flag
    d : u8, // Binary Encoded Decimal Flag
    i : u8, // Interrupt Disable Flag
    z : u8, // Zero Flag
    c : u8, // Carry Flag

    // Onboard Memory
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

pub fn new() -> CPU {
    CPU{
        pc : 0, sp : 0, a : 0, x : 0, y : 0,
        n : 0, v : 0, b : 0, d : 0, i : 0, z : 0, c : 0,
        mem: [0 ; rs6502::MAX_MEM],
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

#[allow(dead_code)]
const INSTRUCTION_UNDEFINED : Instruction = Instruction{mnem : InstructionMnemonic::InstrUndefined, length : 0, cycles : 0 , mode : AddressingMode::AddrModeUndefined};

// Todo: later should be [Instruction, rs6005::NUM_INSTR]
#[allow(dead_code)]
const INSTRUCTION_MATRIX : [Instruction; 0x81] = [
    /* 00 */Instruction{ mnem : InstructionMnemonic::InstrBRK , length : 1, cycles : 7, mode : AddressingMode::AddrModeImplied},
    /* 01 */Instruction{ mnem : InstructionMnemonic::InstrORA , length : 2, cycles : 6, mode : AddressingMode::AddrModeIndX},
    /* 02 */INSTRUCTION_UNDEFINED,
    /* 03 */INSTRUCTION_UNDEFINED,
    /* 04 */INSTRUCTION_UNDEFINED,
    /* 05 */Instruction{ mnem : InstructionMnemonic::InstrORA , length : 2, cycles : 3, mode : AddressingMode::AddrModeZP},
    /* 06 */Instruction{ mnem : InstructionMnemonic::InstrASL , length : 2, cycles : 5, mode : AddressingMode::AddrModeZP},
    /* 07 */INSTRUCTION_UNDEFINED,
    /* 08 */Instruction{ mnem : InstructionMnemonic::InstrPHP , length : 1, cycles : 3, mode : AddressingMode::AddrModeImplied},
    /* 09 */Instruction{ mnem : InstructionMnemonic::InstrORA , length : 2, cycles : 2, mode : AddressingMode::AddrModeImmed},
    /* 0A */Instruction{ mnem : InstructionMnemonic::InstrASL , length : 1, cycles : 2, mode : AddressingMode::AddrModeA},
    /* 0B */INSTRUCTION_UNDEFINED,
    /* 0C */INSTRUCTION_UNDEFINED,
    /* 0D */Instruction{ mnem : InstructionMnemonic::InstrORA , length : 3, cycles : 4, mode : AddressingMode::AddrModeABS},
    /* 0E */Instruction{ mnem : InstructionMnemonic::InstrASL , length : 3, cycles : 6, mode : AddressingMode::AddrModeImmed},
    /* 0F */INSTRUCTION_UNDEFINED,
    /* 10 */Instruction{ mnem : InstructionMnemonic::InstrBPL , length : 2, cycles : 2, mode : AddressingMode::AddrModeRelative},
    /* 11 */Instruction{ mnem : InstructionMnemonic::InstrORA , length : 2, cycles : 5, mode : AddressingMode::AddrModeIndY},
    /* 12 */INSTRUCTION_UNDEFINED,
    /* 13 */INSTRUCTION_UNDEFINED,
    /* 14 */INSTRUCTION_UNDEFINED,
    /* 15 */Instruction{ mnem : InstructionMnemonic::InstrORA , length : 2, cycles : 4, mode : AddressingMode::AddrModeZPX},
    /* 16 */Instruction{ mnem : InstructionMnemonic::InstrASL , length : 2, cycles : 6, mode : AddressingMode::AddrModeZPX},
    /* 17 */INSTRUCTION_UNDEFINED,
    /* 18 */Instruction{ mnem : InstructionMnemonic::InstrCLC , length : 1, cycles : 2, mode : AddressingMode::AddrModeImplied},
    /* 19 */Instruction{ mnem : InstructionMnemonic::InstrORA , length : 3, cycles : 4, mode : AddressingMode::AddrModeABSY},
    /* 1A */INSTRUCTION_UNDEFINED,
    /* 1B */INSTRUCTION_UNDEFINED,
    /* 1C */INSTRUCTION_UNDEFINED,
    /* 1D */Instruction{ mnem : InstructionMnemonic::InstrORA , length : 3, cycles : 4, mode : AddressingMode::AddrModeABSX},
    /* 1E */Instruction{ mnem : InstructionMnemonic::InstrASL , length : 3, cycles : 7, mode : AddressingMode::AddrModeABSX},
    /* 1F */INSTRUCTION_UNDEFINED,
    /* 20 */Instruction{ mnem : InstructionMnemonic::InstrJSR , length : 3, cycles : 6, mode : AddressingMode::AddrModeABS},
    /* 21 */Instruction{ mnem : InstructionMnemonic::InstrAND , length : 2, cycles : 6, mode : AddressingMode::AddrModeIndX},
    /* 22 */INSTRUCTION_UNDEFINED,
    /* 23 */INSTRUCTION_UNDEFINED,
    /* 24 */Instruction{ mnem : InstructionMnemonic::InstrBIT , length : 2, cycles : 3, mode : AddressingMode::AddrModeZP},
    /* 25 */Instruction{ mnem : InstructionMnemonic::InstrAND , length : 2, cycles : 2, mode : AddressingMode::AddrModeZP},
    /* 26 */Instruction{ mnem : InstructionMnemonic::InstrROL , length : 2, cycles : 5, mode : AddressingMode::AddrModeZP},
    /* 27 */INSTRUCTION_UNDEFINED,
    /* 28 */Instruction{ mnem : InstructionMnemonic::InstrPLP , length : 1, cycles : 4, mode : AddressingMode::AddrModeImplied},
    /* 29 */Instruction{ mnem : InstructionMnemonic::InstrAND , length : 2, cycles : 2, mode : AddressingMode::AddrModeImmed},
    /* 2A */Instruction{ mnem : InstructionMnemonic::InstrROL , length : 1, cycles : 2, mode : AddressingMode::AddrModeA},
    /* 2B */INSTRUCTION_UNDEFINED,
    /* 2C */Instruction{ mnem : InstructionMnemonic::InstrBIT , length : 3, cycles : 4, mode : AddressingMode::AddrModeABS},
    /* 2D */Instruction{ mnem : InstructionMnemonic::InstrAND , length : 3, cycles : 4, mode : AddressingMode::AddrModeABS},
    /* 2E */Instruction{ mnem : InstructionMnemonic::InstrROL , length : 3, cycles : 6, mode : AddressingMode::AddrModeABS},
    /* 2F */INSTRUCTION_UNDEFINED,
    /* 30 */Instruction{ mnem : InstructionMnemonic::InstrBMI , length : 2, cycles : 2, mode : AddressingMode::AddrModeRelative},
    /* 31 */Instruction{ mnem : InstructionMnemonic::InstrAND , length : 2, cycles : 5, mode : AddressingMode::AddrModeIndY},
    /* 32 */INSTRUCTION_UNDEFINED,
    /* 33 */INSTRUCTION_UNDEFINED,
    /* 34 */INSTRUCTION_UNDEFINED,
    /* 35 */Instruction{ mnem : InstructionMnemonic::InstrAND , length : 2, cycles : 3, mode : AddressingMode::AddrModeZPX},
    /* 36 */Instruction{ mnem : InstructionMnemonic::InstrROL , length : 2, cycles : 6, mode : AddressingMode::AddrModeZPX},
    /* 37 */INSTRUCTION_UNDEFINED,
    /* 38 */Instruction{ mnem : InstructionMnemonic::InstrSEC , length : 1, cycles : 2, mode : AddressingMode::AddrModeImplied},
    /* 39 */Instruction{ mnem : InstructionMnemonic::InstrAND , length : 3, cycles : 4, mode : AddressingMode::AddrModeABSY},
    /* 3A */INSTRUCTION_UNDEFINED,
    /* 3B */INSTRUCTION_UNDEFINED,
    /* 3C */INSTRUCTION_UNDEFINED,
    /* 3D */Instruction{ mnem : InstructionMnemonic::InstrAND , length : 3, cycles : 4, mode : AddressingMode::AddrModeABSX},
    /* 3E */Instruction{ mnem : InstructionMnemonic::InstrROL , length : 3, cycles : 7, mode : AddressingMode::AddrModeABSX},
    /* 3F */INSTRUCTION_UNDEFINED,
    /* 40 */Instruction{ mnem : InstructionMnemonic::InstrRTI , length : 1, cycles : 6, mode : AddressingMode::AddrModeImplied},
    /* 41 */Instruction{ mnem : InstructionMnemonic::InstrEOR , length : 2, cycles : 6, mode : AddressingMode::AddrModeIndX},
    /* 42 */INSTRUCTION_UNDEFINED,
    /* 43 */INSTRUCTION_UNDEFINED,
    /* 44 */INSTRUCTION_UNDEFINED,
    /* 45 */Instruction{ mnem : InstructionMnemonic::InstrEOR , length : 2, cycles : 3, mode : AddressingMode::AddrModeZP},
    /* 46 */Instruction{ mnem : InstructionMnemonic::InstrLSR , length : 2, cycles : 5, mode : AddressingMode::AddrModeZP},
    /* 47 */INSTRUCTION_UNDEFINED,
    /* 48 */Instruction{ mnem : InstructionMnemonic::InstrPHA , length : 1, cycles : 3, mode : AddressingMode::AddrModeImplied},
    /* 49 */Instruction{ mnem : InstructionMnemonic::InstrEOR , length : 2, cycles : 2, mode : AddressingMode::AddrModeImmed},
    /* 4A */Instruction{ mnem : InstructionMnemonic::InstrLSR , length : 1, cycles : 2, mode : AddressingMode::AddrModeA},
    /* 4B */INSTRUCTION_UNDEFINED,
    /* 4C */Instruction{ mnem : InstructionMnemonic::InstrJMP , length : 3, cycles : 3, mode : AddressingMode::AddrModeABS},
    /* 4D */Instruction{ mnem : InstructionMnemonic::InstrEOR , length : 3, cycles : 4, mode : AddressingMode::AddrModeABS},
    /* 4E */Instruction{ mnem : InstructionMnemonic::InstrLSR , length : 3, cycles : 6, mode : AddressingMode::AddrModeABS},
    /* 4F */INSTRUCTION_UNDEFINED,
    /* 50 */Instruction{ mnem : InstructionMnemonic::InstrBVC , length : 2, cycles : 2, mode : AddressingMode::AddrModeRelative},
    /* 51 */Instruction{ mnem : InstructionMnemonic::InstrEOR , length : 2, cycles : 5, mode : AddressingMode::AddrModeIndY},
    /* 52 */INSTRUCTION_UNDEFINED,
    /* 53 */INSTRUCTION_UNDEFINED,
    /* 54 */INSTRUCTION_UNDEFINED,
    /* 55 */Instruction{ mnem : InstructionMnemonic::InstrEOR , length : 2, cycles : 4, mode : AddressingMode::AddrModeZPX},
    /* 56 */Instruction{ mnem : InstructionMnemonic::InstrLSR , length : 2, cycles : 6, mode : AddressingMode::AddrModeZPX},
    /* 57 */INSTRUCTION_UNDEFINED,
    /* 58 */Instruction{ mnem : InstructionMnemonic::InstrCLI , length : 1, cycles : 2, mode : AddressingMode::AddrModeImplied},
    /* 59 */Instruction{ mnem : InstructionMnemonic::InstrEOR , length : 3, cycles : 4, mode : AddressingMode::AddrModeABSY},
    /* 5A */INSTRUCTION_UNDEFINED,
    /* 5B */INSTRUCTION_UNDEFINED,
    /* 5C */INSTRUCTION_UNDEFINED,
    /* 5D */Instruction{ mnem : InstructionMnemonic::InstrEOR , length : 3, cycles : 4, mode : AddressingMode::AddrModeABSX},
    /* 5E */Instruction{ mnem : InstructionMnemonic::InstrLSR , length : 3, cycles : 7, mode : AddressingMode::AddrModeABSX},
    /* 5F */INSTRUCTION_UNDEFINED,
    /* 60 */Instruction{ mnem : InstructionMnemonic::InstrRTS , length : 1, cycles : 6, mode : AddressingMode::AddrModeImplied},
    /* 61 */Instruction{ mnem : InstructionMnemonic::InstrADC , length : 2, cycles : 6, mode : AddressingMode::AddrModeIndX},
    /* 62 */INSTRUCTION_UNDEFINED,
    /* 63 */INSTRUCTION_UNDEFINED,
    /* 64 */INSTRUCTION_UNDEFINED,
    /* 65 */Instruction{ mnem : InstructionMnemonic::InstrADC , length : 2, cycles : 3, mode : AddressingMode::AddrModeZP},
    /* 66 */Instruction{ mnem : InstructionMnemonic::InstrROR , length : 2, cycles : 5, mode : AddressingMode::AddrModeZP},
    /* 67 */INSTRUCTION_UNDEFINED,
    /* 68 */Instruction{ mnem : InstructionMnemonic::InstrPLA , length : 1, cycles : 4, mode : AddressingMode::AddrModeImplied},
    /* 69 */Instruction{ mnem : InstructionMnemonic::InstrADC , length : 2, cycles : 2, mode : AddressingMode::AddrModeImmed},
    /* 6A */Instruction{ mnem : InstructionMnemonic::InstrROR , length : 1, cycles : 2, mode : AddressingMode::AddrModeA},
    /* 6B */INSTRUCTION_UNDEFINED,
    /* 6C */Instruction{ mnem : InstructionMnemonic::InstrJMP , length : 3, cycles : 5, mode : AddressingMode::AddrModeIndirect},
    /* 6D */Instruction{ mnem : InstructionMnemonic::InstrADC , length : 3, cycles : 4, mode : AddressingMode::AddrModeABS},
    /* 6E */Instruction{ mnem : InstructionMnemonic::InstrROR , length : 3, cycles : 6, mode : AddressingMode::AddrModeABS},
    /* 6F */INSTRUCTION_UNDEFINED,
    /* 70 */Instruction{ mnem : InstructionMnemonic::InstrBVS , length : 2, cycles : 2, mode : AddressingMode::AddrModeRelative},
    /* 71 */Instruction{ mnem : InstructionMnemonic::InstrADC , length : 2, cycles : 5, mode : AddressingMode::AddrModeIndY},
    /* 72 */INSTRUCTION_UNDEFINED,
    /* 73 */INSTRUCTION_UNDEFINED,
    /* 74 */INSTRUCTION_UNDEFINED,
    /* 75 */Instruction{ mnem : InstructionMnemonic::InstrADC , length : 2, cycles : 4, mode : AddressingMode::AddrModeZPX},
    /* 76 */Instruction{ mnem : InstructionMnemonic::InstrROR , length : 2, cycles : 6, mode : AddressingMode::AddrModeZPX},
    /* 77 */INSTRUCTION_UNDEFINED,
    /* 78 */Instruction{ mnem : InstructionMnemonic::InstrSEI , length : 1, cycles : 2, mode : AddressingMode::AddrModeImplied},
    /* 79 */Instruction{ mnem : InstructionMnemonic::InstrADC , length : 3, cycles : 4, mode : AddressingMode::AddrModeABSY},
    /* 7A */INSTRUCTION_UNDEFINED,
    /* 7B */INSTRUCTION_UNDEFINED,
    /* 7C */INSTRUCTION_UNDEFINED,
    /* 7D */Instruction{ mnem : InstructionMnemonic::InstrADC , length : 3, cycles : 4, mode : AddressingMode::AddrModeABSX},
    /* 7E */Instruction{ mnem : InstructionMnemonic::InstrROR , length : 3, cycles : 7, mode : AddressingMode::AddrModeABSX},
    /* 7F */INSTRUCTION_UNDEFINED,
    /* 80 */INSTRUCTION_UNDEFINED,
];