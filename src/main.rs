use std::io::{stdin, stdout, Write};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;
use std::{fs::File, io::Read, path::Path};

use clap::{App, Arg, ArgMatches};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use emulator::{DeviceRegister, Emulator};

mod emulator;

/// Get matched arguments
fn build_command_line<'a>() -> ArgMatches<'a> {
    App::new("lc3emu")
        .version("1.0")
        .author("Gil Mendes <gil00mendes@gmail.com>")
        .about("A LC3 emulator written in Rust")
        .arg(
            Arg::with_name("ROM")
                .help("ROM to be executed")
                .required(true)
                .index(1),
        )
        .get_matches()
}

/// Read ROM content as 8-bit integer vector
fn read_room<P: AsRef<Path>>(path: P) -> Vec<u8> {
    // read file
    let mut file = File::open(path).unwrap();

    // create a new vector to hold the instructions
    let mut file_buffer = Vec::new();

    // get the file content into the buffer
    file.read_to_end(&mut file_buffer).unwrap();
    file_buffer
}

fn handle_io_events(ui_tx: Sender<(DeviceRegister, u16)>, ui_rx: Receiver<(DeviceRegister, u16)>) {
    let stdin = stdin();
    let stdin = stdin.lock();
    let mut bytes = stdin.bytes();

    'main: loop {
        // check if there is a new event to be processed
        let event = ui_rx.recv().unwrap();

        match event {
            (DeviceRegister::DisplayData, ch) => {
                print!("{}", ch as u8 as char);
                stdout().flush().unwrap();

                // inform the CPU that we are ready to receive a new char
                ui_tx
                    .send((DeviceRegister::DisplayStatus, 1 << 15))
                    .unwrap();
            }
            (DeviceRegister::KeyboardStatus, _) => {
                // disable canonical and echo modes
                enable_raw_mode().unwrap();

                let ch = bytes.next().unwrap().unwrap() as u16;
                ui_tx.send((DeviceRegister::KeyboardData, ch)).unwrap();

                // restore default terminal settings
                disable_raw_mode().unwrap();
            }
            (DeviceRegister::MachineControl, _) => {
                break 'main;
            }
            _ => {
                thread::sleep(Duration::from_millis(30));
            }
        }
    }
}

fn main() {
    // build command line and get the matched arguments
    let matches = build_command_line();

    // read file content into a vector
    let rom_path = matches.value_of("ROM").unwrap();
    let rom_data = read_room(rom_path);

    // create a new messaging passing channel to get notified about I/O events. Since we need bidirectional
    // communication we have two two different channels.
    let (emu_tx, ui_rx) = mpsc::channel::<(DeviceRegister, u16)>();
    let (ui_tx, emu_rx) = mpsc::channel::<(DeviceRegister, u16)>();

    // start UI thread
    let th_handler = thread::spawn(move || handle_io_events(ui_tx, ui_rx));

    // Build and start the emulator
    let mut emulator = Emulator::new(rom_data);
    emulator.execute(emu_tx, emu_rx);

    th_handler.join().unwrap();
}
