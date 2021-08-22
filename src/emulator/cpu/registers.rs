//! CPU registers for the LC-3 emulator

/// LC-3 CPU condition flags
#[derive(Default, Debug)]
pub struct Flags {
    pub negative: bool,
    pub zero: bool,
    pub positive: bool,
}

impl Flags {
    /// Update flags based on the given value
    pub fn update(&mut self, value: u16) {
        // reset all flags
        self.negative = false;
        self.zero = false;
        self.positive = false;

        if value == 0 {
            self.zero = true
        } else if (value >> 15) != 0 {
            self.negative = true;
        } else {
            self.positive = true;
        }
    }
}

/// LC-3 CPU registers
#[derive(Default)]
pub struct Registers {
    /// General purpose register 0
    pub r0: u16,
    /// General purpose register 1
    pub r1: u16,
    /// General purpose register 2
    pub r2: u16,
    /// General purpose register 3
    pub r3: u16,
    /// General purpose register 4
    pub r4: u16,
    /// General purpose register 5
    pub r5: u16,
    /// General purpose register 6
    pub r6: u16,
    /// General purpose register 7
    pub r7: u16,
    /// Program counter
    pub pc: u16,
    // Condition flags
    pub flags: Flags,
}

impl Registers {
    /// Read a register state by its index
    pub fn get(&self, index: u16) -> u16 {
        match index {
            0 => self.r0,
            1 => self.r1,
            2 => self.r2,
            3 => self.r3,
            4 => self.r4,
            5 => self.r5,
            6 => self.r6,
            7 => self.r7,
            _ => panic!("Registers: index out of bound"),
        }
    }

    /// Set a register state by its index
    pub fn set(&mut self, index: u16, value: u16) {
        match index {
            0 => self.r0 = value,
            1 => self.r1 = value,
            2 => self.r2 = value,
            3 => self.r3 = value,
            4 => self.r4 = value,
            5 => self.r5 = value,
            6 => self.r6 = value,
            7 => self.r7 = value,
            _ => panic!("Registers: index out of bound"),
        }
    }
}
