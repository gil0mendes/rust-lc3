//! CPU instructions declaration and  decoder

/// LC-3 Instructions
#[derive(PartialEq, Debug)]
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

#[cfg(test)]
mod instructions_test {
    use super::*;

    #[test]
    fn instructions_initial_values() {
        assert_eq!(Some(Instructions::BR), Instructions::get(0));
        assert_eq!(Some(Instructions::ADD), Instructions::get(1));
        assert_eq!(Some(Instructions::LD), Instructions::get(2));
        assert_eq!(Some(Instructions::ST), Instructions::get(3));
        assert_eq!(Some(Instructions::JSR), Instructions::get(4));
        assert_eq!(Some(Instructions::AND), Instructions::get(5));
        assert_eq!(Some(Instructions::LDR), Instructions::get(6));
        assert_eq!(Some(Instructions::STR), Instructions::get(7));
        assert_eq!(Some(Instructions::RTI), Instructions::get(8));
        assert_eq!(Some(Instructions::NOT), Instructions::get(9));
        assert_eq!(Some(Instructions::LDI), Instructions::get(10));
        assert_eq!(Some(Instructions::STI), Instructions::get(11));
        assert_eq!(Some(Instructions::JMP), Instructions::get(12));
        assert_eq!(Some(Instructions::RES), Instructions::get(13));
        assert_eq!(Some(Instructions::LEA), Instructions::get(14));
        assert_eq!(Some(Instructions::TRAP), Instructions::get(15));
    }
}
