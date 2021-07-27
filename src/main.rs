use std::{fs::File, io::Read, path::Path, u16};

use clap::{App, Arg, ArgMatches};
use emulator::Emulator;

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

fn main() {
    // build command line and get the matched arguments
    let matches = build_command_line();

    // read file content into a vector
    let rom_path = matches.value_of("ROM").unwrap();
    let rom_data = read_room(rom_path);

    // Build and start the emulator
    let mut emulator = Emulator::new(rom_data);
    emulator.execute();
}
