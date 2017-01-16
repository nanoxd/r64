// TODO: Better name
#[derive(Debug)]
enum Ep {
    D,
    DxxDxx,
    RFU,
}

impl Default for Ep {
    fn default() -> Ep { Ep::D }
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
    ep: Ep,
    be: RegConfigBe,
}

impl RegConfig {
    pub fn power_on_reset(&mut self) {
        self.ep = Ep::D;
        self.be = RegConfigBe::BigEndian;
    }
}
