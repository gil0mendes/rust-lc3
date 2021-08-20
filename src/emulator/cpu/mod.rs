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

    pub fn next_tick(&mut self, memory: &mut Memory) {
        // get the next instruction in memory
        let instruction_raw = memory.read(self.registers.pc);

        // increment PC
        self.registers.pc += 1;

        // process next opcode
        self.next_opcode(instruction_raw, memory);
    }

    /// Process the next opcode
    pub fn next_opcode(&mut self, instruction: u16, memory: &mut Memory) {
        let opcode_raw = instruction >> 12;
        let opcode = Instructions::get(opcode_raw);

        match opcode {
            Some(Instructions::BR) => self.opcode_br(instruction),
            Some(Instructions::ADD) => self.opcode_add(instruction),
            Some(Instructions::AND) => self.opcode_and(instruction),
            Some(Instructions::JMP) => self.opcode_jmp_ret(instruction),
            Some(Instructions::JSR) => self.opcode_jsr(instruction),
            Some(Instructions::LD) => self.opcode_ld(instruction, memory),
            Some(Instructions::LDI) => self.opcode_ldi(instruction, memory),
            Some(Instructions::LDR) => self.opcode_ldr(instruction, memory),
            Some(Instructions::LEA) => self.opcode_lea(instruction),
            Some(Instructions::NOT) => self.opcode_not(instruction),
            Some(Instructions::ST) => self.opcode_st(instruction, memory),
            Some(Instructions::STI) => self.opcode_sti(instruction, memory),
            Some(Instructions::STR) => self.opcode_str(instruction, memory),
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

    /// ADD operator
    ///
    /// If bit [5] is 0, the second source operand is obtained from SR2.
    /// If bit [5] is 1, the second source operand is obtained by sign-extending the imm5 field to 16 bits.
    /// In both cases, the second source operand is added to the contents of SR1 and the result stored in DR. The
    /// condition codes are set, based on whether the result is negative, zero, or positive.
    fn opcode_add(&mut self, instruction: u16) {
        let dest = (instruction >> 9) & 0x7;
        let src1 = (instruction >> 6) & 0x7;
        let is_imm = (instruction >> 5) & 0x1 == 1;

        let new_value = if is_imm {
            let src2 = self.sign_extend(instruction & 0x1F, 5);
            self.registers.get(src1) + src2
        } else {
            let src2 = instruction & 0x7;
            self.registers.get(src1) + self.registers.get(src2)
        };

        self.registers.set(dest, new_value);
        self.registers.flags.update(new_value);
    }

    /// BR operator
    ///
    /// The condition codes specified by the state of bits [11:9] are tested.
    /// If bit [11] is set, N is tested;
    /// if bit [11] is clear, N is not tested.
    /// If bit [10] is set, Z is tested, etc.
    /// If any of the condition codes tested is set, the program branches to the location specified by adding the
    /// sign-extended PCoffset9 field to the incremented PC.
    fn opcode_br(&mut self, instruction: u16) {
        let offset = instruction & 0xFF;
        let flag_n = (instruction >> 9) & 0x1 == 1;
        let flag_z = (instruction >> 10) & 0x1 == 1;
        let flag_p = (instruction >> 11) & 0x1 == 1;

        let cpu_flags = &self.registers.flags;
        if (flag_n && cpu_flags.negative)
            || (flag_z && cpu_flags.zero)
            || (flag_p && cpu_flags.positive)
        {
            self.registers.pc += offset;
        }
    }

    /// JMP/RET operator
    ///
    /// The program unconditionally jumps to the location specified by the contents of the base register. Bits [8:6]
    /// identify the base register.
    ///
    /// The RET instruction is a special case of the JMP instruction. The PC is loaded with the contents of R7, which
    /// contains the linkage back to the instruction following the subroutine call instruction.
    fn opcode_jmp_ret(&mut self, instruction: u16) {
        let base_addr = (instruction >> 6) & 0x7;

        if base_addr == 0x7 {
            self.registers.pc = self.registers.r7;
        } else {
            self.registers.pc = base_addr;
        }
    }

    /// JSR/JSRR operator
    ///
    /// First, the incremented PC is saved in R7. This is the linkage back to the calling routine.
    /// Then the PC is loaded with the address of the first instruction of the subroutine, causing an unconditional jump
    /// to that address.
    /// The address of the subroutine is obtained from the base register (if bit [11] is 0), or the address is computed
    /// by sign-extending bits [10:0] and adding this value to the incremented PC (if bit [11] is 1).
    fn opcode_jsr(&mut self, instruction: u16) {
        let is_absolute_addr = (instruction >> 11) & 0x1 == 1;

        self.registers.r7 = self.registers.pc;

        if is_absolute_addr {
            let offset = instruction & 0x7ff;
            self.registers.pc = self.registers.pc + self.sign_extend(offset, 11);
        } else {
            self.registers.pc = (instruction >> 6) & 0x7;
        }
    }

    /// LD operator
    ///
    /// An address is computed by sign-extending bits [8:0] to 16 bits and adding this value to the incremented PC.
    /// The contents of memory at this address are loaded into DR.
    /// The condition codes are set, based on whether the value loaded is negative, zero, or positive.
    fn opcode_ld(&mut self, instruction: u16, memory: &Memory) {
        let target_index = (instruction >> 9) & 0x7;
        let offset = instruction & 0xFF;

        let address = self.registers.pc + self.sign_extend(offset, 9);
        let value = memory.read(address);
        self.registers.set(target_index, value);

        self.registers.flags.update(value);
    }

    /// LDI operator
    ///
    /// An address is computed by sign-extending bits [8:0] to 16 bits and adding this value to the incremented PC.
    /// What is stored in memory at this address is the address of the data to be loaded into DR.
    /// The condition codes are set, based on whether the value loaded is negative, zero, or positive.
    fn opcode_ldi(&mut self, instruction: u16, memory: &Memory) {
        let target_index = (instruction >> 9) & 0x7;
        let offset = instruction & 0xFF;

        let address = self.registers.pc + self.sign_extend(offset, 9);
        let value = memory.read(memory.read(address));
        self.registers.set(target_index, value);

        self.registers.flags.update(value);
    }

    /// LDR operator
    ///
    /// An address is computed by sign-extending bits [5:0] to 16 bits and adding this value to the contents of the
    /// register specified by bits [8:6]. The contents of memory at this address are loaded into DR.
    /// The condition codes are set, based on whether the value loaded is negative, zero, or positive.
    fn opcode_ldr(&mut self, instruction: u16, memory: &Memory) {
        let target_index = (instruction >> 9) & 0x7;
        let base_addr = (instruction >> 6) & 0x7;
        let offset = instruction & 0x3F;

        let value = memory.read(base_addr + self.sign_extend(offset, 6));
        self.registers.set(target_index, value);

        self.registers.flags.update(value);
    }

    /// LEA operator
    ///
    /// An address is computed by sign-extending bits [8:0] to 16 bits and adding this value to the incremented PC.
    /// This address is loaded into DR.
    /// The condition codes are set, based on whether the value loaded is negative, zero, or positive.
    fn opcode_lea(&mut self, instruction: u16) {
        let target_index = (instruction >> 9) & 0x7;
        let offset = instruction & 0xFF;

        let value = self.registers.pc + self.sign_extend(offset, 9);
        self.registers.set(target_index, value);

        self.registers.flags.update(value);
    }

    /// NOT operator
    ///
    /// The bit-wise complement of the contents of SR is stored in DR.
    /// The condition codes are set, based on whether the binary value produced, taken as a 2’s complement integer, is
    /// negative, zero, or positive.
    fn opcode_not(&mut self, instruction: u16) {
        let target_index = (instruction >> 9) & 0x7;
        let src_index = (instruction >> 6) & 0x7;

        self.registers
            .set(target_index, !self.registers.get(src_index));
    }

    /// ST operator
    ///
    /// The contents of the register specified by SR are stored in the memory location whose address is computed by
    /// sign-extending bits [8:0] to 16 bits and adding this value to the incremented PC.
    fn opcode_st(&mut self, instruction: u16, memory: &mut Memory) {
        let src_index = (instruction >> 9) & 0x7;
        let offset = self.sign_extend(instruction & 0x1FF, 9);

        memory.write(self.registers.pc + offset, self.registers.get(src_index));
    }

    /// STI operator
    ///
    /// The contents of the register specified by SR are stored in the memory location whose address is obtained as
    /// follows: Bits [8:0] are sign-extended to 16 bits and added to the incremented PC. What is in memory at this
    /// address is the address of the location to which the data in SR is stored.
    fn opcode_sti(&mut self, instruction: u16, memory: &mut Memory) {
        let src_index = (instruction >> 9) & 0x7;
        let offset = self.sign_extend(instruction & 0x1FF, 9);

        memory.write(
            memory.read(self.registers.pc + offset),
            self.registers.get(src_index),
        );
    }

    /// STR operator
    ///
    /// The contents of the register specified by SR are stored in the memory location whose address is computed by
    /// sign-extending bits [5:0] to 16 bits and adding this value to the contents of the register specified by bits
    /// [8:6].
    fn opcode_str(&mut self, instruction: u16, memory: &mut Memory) {
        let src_index = (instruction >> 9) & 0x7;
        let base_addr = (instruction >> 6) & 0x7;
        let offset = self.sign_extend(instruction & 0x3F, 6);

        memory.write(base_addr + offset, self.registers.get(src_index));
    }
}
