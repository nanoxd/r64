// TODO: Better name
#[derive(Debug)]
enum DataTransferPattern {
    D,
    DxxDxx,
}

impl Default for DataTransferPattern {
    fn default() -> Self { DataTransferPattern::D }
}

#[derive(Debug)]
enum Endianess {
    Little,
    Big,
}

impl Default for Endianess {
    fn default() -> Self {
         Endianess::Big
    }
}

#[derive(Debug, Default)]
pub struct RegConfig {
    transfer_data_pattern: DataTransferPattern,
    endianess: Endianess,
}

impl RegConfig {
    pub fn power_on_reset(&mut self) {
        self.transfer_data_pattern = DataTransferPattern::D;
        self.endianess = Endianess::Big;
    }
}

impl From<u32> for RegConfig {
    fn from(value: u32) -> Self {
        unimplemented!();
        // RegConfig {
        // }
    }
}
