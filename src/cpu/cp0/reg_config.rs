// TODO: Better name
#[derive(Debug)]
enum RegConfigEp {
    D,
    DxxDxx,
    RFU,
}

impl Default for RegConfigEp {
    fn default() -> RegConfigEp { RegConfigEp::D }
}

#[derive(Debug)]
enum RegConfigBe {
    LittleEndian,
    BigEndian,
}

impl Default for RegConfigBe {
    fn default() -> RegConfigBe { RegConfigBe::BigEndian }
}

#[derive(Debug, Default)]
pub struct RegConfig {
    reg_config_ep: RegConfigEp,
    reg_config_be: RegConfigBe,
}

impl RegConfig {
    pub fn power_on_reset(&mut self) {
        self.reg_config_ep = RegConfigEp::D;
        self.reg_config_be = RegConfigBe::BigEndian;
    }
}
