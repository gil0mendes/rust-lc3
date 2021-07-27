use self::{
    cpu::CPU,
    memory::{Memory, MEMORY_SIZE},
};

mod cpu;
mod memory;

pub struct Emulator {
    memory: Memory,
    cpu: CPU,
}

impl Emulator {
    pub fn new(binary_data: Vec<u8>) -> Self {
        let mut emulator = Self {
            memory: Memory::new(),
            cpu: CPU::new(),
        };

        // first 16 bit entry is the base address here the binary must be loaded
        let mut address = (binary_data[0] as u16) << 8 | (binary_data[1] as u16);

        // load the rest of the binary on the memory
        let mut i = 2;
        let limit = Vec::len(&binary_data);
        while i + 1 <= limit {
            let data = (binary_data[i] as u16) << 8 | (binary_data[i + 1] as u16);
            emulator.memory.write(address, data);

            i += 2;
            address += 1;
        }

        emulator
    }

    /// Initiate the execution loop
    pub fn execute(&mut self) {
        loop {
            self.cpu.next_tick(&mut self.memory);

            if self.cpu.get_registers().pc >= MEMORY_SIZE as u16 {
                println!("Emulator: out of memory bound");
                break;
            }
        }
    }
}
