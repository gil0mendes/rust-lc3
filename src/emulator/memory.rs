//! LC-3 has a memory address space of 2^16 locations, so which gives 65 536 possible locations (u16 max capacity),
//! in each location is possible to store a 16-bit value. This means that in total is possible to store 128kb.

use std::usize;

/// Represents the size of a LC-3 memory.
const MEMORY_SIZE: usize = u16::MAX as usize;

pub struct Memory {
    /// Memory is a vector of 65_536 positions
    cells: [u16; MEMORY_SIZE],
}
