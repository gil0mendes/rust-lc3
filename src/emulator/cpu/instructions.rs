//! CPU instructions declaration and  decoder

/// LC-3 Instructions
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
