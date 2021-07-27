///! Constrains the code to implement the Control Unit and ALU of the LC-3 machine.
use self::{instructions::Instructions, registers::Registers};

use super::memory::Memory;

mod instructions;
mod registers;

/// This is the default PC value when the CPU starts
const DEFAULT_PC: u16 = 0x3000;

/// LC-3 CPU
pub struct CPU {
    /// CPU registers and flags
    registers: Registers,
}

impl CPU {
    /// Create a new CPU instance
    pub fn new() -> Self {
        let mut cpu = Self {
            registers: Registers::default(),
        };

        // define default PC address
        cpu.registers.pc = DEFAULT_PC;

        cpu
    }

    /// Borrow CPU registers to external consult
    pub fn get_registers(&self) -> &Registers {
        &self.registers
    }

    pub fn next_tick(&mut self, memory: &Memory) {
        // get the next instruction in memory
        let instruction_raw = memory.read(self.registers.pc);

        // increment PC
        self.registers.pc += 1;

        // process next opcode
        self.next_opcode(instruction_raw);
    }

    /// Process the next opcode
    pub fn next_opcode(&mut self, instruction: u16) {
        let opcode_raw = instruction >> 12;
        let opcode = Instructions::get(opcode_raw);

        match opcode {
            Some(Instructions::AND) => self.opcode_and(instruction),
            _ => panic!("CPU: instruction ({:?}) not implemented", opcode.unwrap()),
        };
    }

    /// Sign-extend a small value into a 16-bit one using two's complements
    fn sign_extend(&self, mut value: u16, num_bits: u16) -> u16 {
        if (value >> (num_bits - 1)) & 1 != 0 {
            value |= 0xFFFF << num_bits
        }

        value
    }

    /// AND operator
    ///
    /// If bit [5] is 0, the second source operand is obtained from SR2.
    /// If bit [5] is 1, the second source operand is obtained by sign-extending the imm5 field to 16 bits.
    /// In both cases, the second source operand is added to the contents of SR1 and the result stored in DR. The
    /// condition codes are set, based on whether the result is negative, zero, or positive.
    fn opcode_and(&mut self, instruction: u16) {
        let dest = (instruction >> 9) & 0x7;
        let src1 = (instruction >> 6) & 0x7;
        let is_imm = (instruction >> 5) & 0x1 == 1;

        let new_value = if is_imm {
            let src2 = self.sign_extend(instruction & 0x1F, 5);
            self.registers.get(src1) & src2
        } else {
            let src2 = instruction & 0x7;
            self.registers.get(src1) & self.registers.get(src2)
        };

        self.registers.set(dest, new_value);
        self.registers.flags.update(new_value);
    }
}
