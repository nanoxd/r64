use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

struct CPU {

}

impl CPU {
    fn new() -> CPU {
        CPU { }
    }
}

fn main() {
    let pif_file_name = env::args().nth(1).unwrap();
    let rom_file_name = env::args().nth(2).unwrap();

    let pif = read_bin(pif_file_name);
    let rom = read_bin(rom_file_name);

    let mut cpu = CPU::new();
}

fn read_bin<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file = File::open(path.as_ref()).unwrap();
    let mut file_buffer = Vec::new();
    file.read_to_end(&mut file_buffer).unwrap();

    file_buffer
}
