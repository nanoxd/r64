// TODO: Better name
#[derive(Debug)]
enum DataTransferPattern {
    D,
    DxxDxx,
}

impl Default for DataTransferPattern {
    fn default() -> Self {
         DataTransferPattern::D
    }
}

impl From<u32> for DataTransferPattern {
    fn from(value: u32) -> Self {
        match (value >> 24) & 0b1111 {
            0 => DataTransferPattern::D,
            6 => DataTransferPattern::DxxDxx,
            _ => panic!("Invalid data transfer pattern (EP): {:#x}", value)
        }
    }
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

impl From<u32> for Endianess {
    fn from(value: u32) -> Self {
        match (value >> 15) & 0b1 {
            0 => Endianess::Little,
            1 => Endianess::Big,
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Default)]
pub struct RegConfig {
    data_transfer_pattern: DataTransferPattern,
    endianess: Endianess,
    cu: bool,
    kseg0_cache_enable_bits: [bool; 3],
}

impl RegConfig {
    fn kseg0_cache_enabled(&self) -> bool {
        !(!self.kseg0_cache_enable_bits[0] &&
        self.kseg0_cache_enable_bits[1] &&
        !self.kseg0_cache_enable_bits[2])
    }
}

impl From<u32> for RegConfig {
    fn from(value: u32) -> Self {
        RegConfig {
            data_transfer_pattern: value.into(),
            endianess: value.into(),

            cu: (value & (1 << 3)) != 0,
            kseg0_cache_enable_bits: [
                (value & (1 << 0)) != 0,
                (value & (1 << 1)) != 0,
                (value & (1 << 2)) != 0,
            ],
        }
    }
}
