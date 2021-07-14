//! CPU registers for the LC-3 emulator

/// LC-3 CPU condition flags
pub struct Flags {
    negative: bool,
    zero: bool,
    positive: bool,
}

/// LC-3 CPU registers
pub struct Registers {
    /// General purpose register 0
    r0: u16,
    /// General purpose register 1
    r1: u16,
    /// General purpose register 2
    r2: u16,
    /// General purpose register 3
    r3: u16,
    /// General purpose register 4
    r4: u16,
    /// General purpose register 5
    r5: u16,
    /// General purpose register 6
    r6: u16,
    /// General purpose register 7
    r7: u16,
    /// Program counter
    pc: u16,
    // Condition flags
    flags: Flags,
}
