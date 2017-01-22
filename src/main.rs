extern crate byteorder;
#[macro_use]
extern crate enum_primitive;
extern crate num;

mod cpu;
mod interconnect;
mod peripheral_interface;
mod n64;
mod mem_map;
mod rsp;

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let pif_file_name = env::args().nth(1).unwrap();
    let rom_file_name = env::args().nth(2).unwrap();

    let pif = read_bin(pif_file_name);
    let rom = read_bin(rom_file_name);

    let mut n64 = n64::N64::new(pif);

    loop {
        // println!("N64: {:#?}", &n64);
        n64.run_instruction();
    }
}

fn read_bin<P: AsRef<Path>>(path: P) -> Box<[u8]> {
    let mut file = File::open(path.as_ref()).unwrap();
    let mut file_buffer = Vec::new();
    file.read_to_end(&mut file_buffer).unwrap();

    file_buffer.into_boxed_slice()
}
