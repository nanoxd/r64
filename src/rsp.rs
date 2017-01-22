pub struct RSP {
    halt: bool,
    interrupt_enable: bool,
    broke: bool,
}

impl RSP {
    pub fn new() -> RSP {
        RSP {
            halt: true,
            interrupt_enable: false,
            broke: false
        }
    }

    pub fn read_status_reg(&self) -> u32 {
        (if self.halt { 1 } else { 0 } << 0) |
        (if self.interrupt_enable { 1 } else { 0 } << 1)
    }

    pub fn write_status_reg(&mut self, value: u32) {
        // TODO: Add remaining bits
        if (value & (1 << 0)) != 0 {
            self.halt = false
        }

        if (value & (1 << 1)) != 0 {
            self.halt = true
        }

        if (value & (1 << 2)) != 0 {
            self.broke = false
        }

        if (value & (1 << 3)) != 0 {
            self.interrupt_enable = false
        }
        if (value & 0xfffffff0) != 0 {
            panic!("Writes to unsupported RSP status reg: {:#018X}", value);
        }
    }

    pub fn read_dma_busy_reg(&self) -> u32 {
        0
    }

    pub fn write_dma_busy_reg(&self, value: u32) {
        panic!("Attempted write to sp_dma_busy_reg: {:#?}", value)
    }
}
