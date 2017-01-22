#[derive(Default)]
pub struct VideoInterface {
    interrupt_half_line: u32,
}

impl VideoInterface {
    pub fn read_vi_intr_reg(&self) -> u32 {
        self.interrupt_half_line
    }

    pub fn write_vi_intr_reg(&mut self, value: u32) {
        self.interrupt_half_line = value & 0x0000_03ff;
    }
}
