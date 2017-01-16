const RAM_SIZE: usize = 4 * 1024 * 1024;

pub struct Interconnect {
    ram: Box<[u16; RAM_SIZE]>,
}

impl Default for Interconnect {
    fn default() -> Interconnect {
        Interconnect {
            ram: Box::new([0; RAM_SIZE])
        }
    }
}
