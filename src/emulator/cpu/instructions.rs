//! CPU instructions declaration and  decoder

/// LC-3 Instructions
#[derive(Debug)]
pub enum Instructions {
    /// branch
    BR,
    /// add
    ADD,
    /// load
    LD,
    /// store
    ST,
    /// jump register
    JSR,
    /// bitwise and
    AND,
    /// load register
    LDR,
    /// store register
    STR,
    /// unused
    RTI,
    /// bitwise not
    NOT,
    /// load indirect
    LDI,
    /// store indirect
    STI,
    /// jump
    JMP,
    /// reserved (unused)
    RES,
    /// load effective address
    LEA,
    /// execute trap
    TRAP,
}

impl Instructions {
    /// Get instruction from u16
    pub fn get(opcode: u16) -> Option<Instructions> {
        match opcode {
            0 => Some(Self::BR),
            1 => Some(Self::ADD),
            2 => Some(Self::LD),
            3 => Some(Self::ST),
            4 => Some(Self::JSR),
            5 => Some(Self::AND),
            6 => Some(Self::LDR),
            7 => Some(Self::STR),
            8 => Some(Self::RTI),
            9 => Some(Self::NOT),
            10 => Some(Self::LDI),
            11 => Some(Self::STI),
            12 => Some(Self::JMP),
            13 => Some(Self::RES),
            14 => Some(Self::LEA),
            15 => Some(Self::TRAP),
            _ => None,
        }
    }
}
