const RAM_SIZE: usize = 4 * 1024 * 1024;

pub struct Interconnect {
    ram: [u16; RAM_SIZE],
}
