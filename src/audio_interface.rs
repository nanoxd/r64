#[derive(Default)]
pub struct AudioInterface {
    dram_addr_reg: u32,
}

impl AudioInterface {
    pub fn read_dram_addr_reg(&self) -> u32 {
        self.dram_addr_reg
    }

    pub fn write_dram_addr_reg(&mut self, value: u32) {
        self.dram_addr_reg = value & 0x00ff_ffff;
    }
    }
}
