pub struct Cpu {
    af: u16,      // Accumulator and flags
    bc: u16,      // General purpose register
    de: u16,      // General purpose register
    hl: u16,      // General purpose register
    sp: u16,      // Stack pointer
    pc: u16,      // Program counter
    flags: Flags, // Current flags
}

struct Flags {
    z: bool, // Zero flag
    n: bool, // Subtract flag
    h: bool, // Half carry flag
    c: bool, // Carry flag
}
