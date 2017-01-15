use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let file_name = env::args().nth(1).unwrap();
}

fn load_rom<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file = File::open(&path).unwrap();
    let mut file_buffer = Vec::new();
    file.read_to_end(&mut file_buffer).unwrap();

    file_buffer
}
