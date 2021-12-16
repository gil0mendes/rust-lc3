use std::{
    sync::mpsc::{Receiver, Sender},
    time::Duration,
};

use self::{
    cpu::CPU,
    memory::{Memory, MEMORY_SIZE},
};

mod cpu;
mod memory;

/// Device registers
#[derive(Debug)]
pub enum DeviceRegister {
    KeyboardStatus = 0xFE00,
    KeyboardData = 0xFF02,
    DisplayStatus = 0xFE04,
    DisplayData = 0xFF06,
    MachineControl = 0xFFFE,
}

/// Structure to help managing events
pub struct Events {
    /// Emulator sender
    emu_tx: Sender<(DeviceRegister, u16)>,
    /// Emulator receiver
    emu_rx: Receiver<(DeviceRegister, u16)>,
}

impl Events {
    pub fn new(
        emu_tx: Sender<(DeviceRegister, u16)>,
        emu_rx: Receiver<(DeviceRegister, u16)>,
    ) -> Self {
        Self { emu_tx, emu_rx }
    }

    /// Send an event to the outside world
    pub fn send(&mut self, event: DeviceRegister, data: u16) {
        self.emu_tx.send((event, data)).unwrap()
    }

    /// Blocks until receives an event from the outside world
    pub fn receive(&self) -> (DeviceRegister, u16) {
        self.emu_rx.recv().unwrap()
    }

    /// Blocks until receives an event from the outside world, but timeouts if it takes more than 10ms
    pub fn receive_timeout(&self) -> Option<(DeviceRegister, u16)> {
        self.emu_rx.recv_timeout(Duration::from_millis(10)).ok()
    }
}

pub struct Emulator {
    memory: Memory,
    cpu: CPU,
}

impl Emulator {
    pub fn new(binary_data: Vec<u8>) -> Self {
        // first 16 bit entry is the base address here the binary must be loaded. This is also the initial address where
        // the program counter must point to.
        let mut address = (binary_data[0] as u16) << 8 | (binary_data[1] as u16);

        let mut emulator = Self {
            memory: Memory::new(),
            cpu: CPU::new(address),
        };

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

    /// Deal I/O events from CPU and from the outside world
    fn check_events(&mut self, events: &mut Events) {
        // check if there is a new event from the outside would
        match events.receive_timeout() {
            Some((DeviceRegister::KeyboardData, val)) => {
                self.memory.write(DeviceRegister::KeyboardData as u16, val);
                self.memory.write(DeviceRegister::KeyboardStatus as u16, 1);
            }
            Some((DeviceRegister::DisplayStatus, _)) => {
                self.memory
                    .write(DeviceRegister::DisplayStatus as u16, 1 << 15);
            }
            _ => {}
        }
    }

    /// Initiate the execution loop
    pub fn execute(
        &mut self,
        emu_tx: Sender<(DeviceRegister, u16)>,
        emu_rx: Receiver<(DeviceRegister, u16)>,
    ) {
        // create a event manager to deal with the events between CPU and World
        let mut events = Events::new(emu_tx, emu_rx);

        // before start executing we need to enable the machine and tell that we can write chars to the screen
        self.memory
            .write(DeviceRegister::MachineControl as u16, 1 << 15);
        self.memory
            .write(DeviceRegister::DisplayStatus as u16, 1 << 15);

        loop {
            self.cpu.next_tick(&mut self.memory, &mut &mut events);

            if self.cpu.get_registers().pc >= MEMORY_SIZE as u16 {
                println!("Emulator: out of memory bound");
                break;
            }

            // when the machine enters a halt state break the cycle
            if self.memory.read(DeviceRegister::MachineControl as u16) == 0 {
                break;
            }

            // on each cycle check if there is some event to be sent
            self.check_events(&mut events);
        }

        // notify that the machine was shutdown
        events.send(DeviceRegister::MachineControl, 1 << 15);
    }
}
