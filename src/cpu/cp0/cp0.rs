use super::reg_config;
use super::reg_status;

#[derive(Debug, Default)]
pub struct CP0 {
    reg_config: reg_config::RegConfig,
    reg_status: reg_status::RegStatus,
}

impl CP0 {
    pub fn write_reg(&mut self, index: u32, data: u64) {
        match index {
            12 => { self.reg_status = (data as u32).into(); },
            16 => { self.reg_config = (data as u32).into(); },
            _ => panic!("Unrecognized Cp0 reg: {}, {:#018x}", index, data)
        }
    }
}
