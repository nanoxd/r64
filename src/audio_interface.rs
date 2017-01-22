#[derive(Default)]
pub struct AudioInterface {
    dram_addr: u32,
}

impl AudioInterface {
    pub fn read_dram_addr(&self) -> u32 {
        self.dram_addr
    }

    pub fn write_dram_addr(&mut self, value: u32) {
        self.dram_addr = value & 0x00ff_ffff;
    }
}
