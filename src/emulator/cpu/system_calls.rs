/// Available system calls
#[derive(Debug)]
pub enum SystemCall {
    /// Get character from keyboard, not eached onto the terminal
    GETC = 0x20,
    /// output a character
    OUT = 0x21,
    /// output a word string
    PUTS = 0x22,
    /// get character from keyboard, echoed onto the terminal
    IN = 0x23,
    /// output a byte string
    PUTSP = 0x24,
    /// halt the program
    HALT = 0x25,
}

impl SystemCall {
    /// Get system call out of trap instruction
    pub fn get(instruction: u16) -> Option<Self> {
        let code = instruction & 0xFF;

        match code {
            0x20 => Some(Self::GETC),
            0x21 => Some(Self::OUT),
            0x22 => Some(Self::PUTS),
            0x23 => Some(Self::IN),
            0x24 => Some(Self::PUTSP),
            0x25 => Some(Self::HALT),
            _ => None,
        }
    }
}
