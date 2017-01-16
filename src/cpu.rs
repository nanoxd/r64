use ::interconnect;

const NUM_GPR: usize = 32;

pub struct CPU {
    reg_grp: [u64; NUM_GPR],
    reg_fpr: [u64; NUM_GPR],

    reg_pc: u64,

    reg_hi: u64,
    reg_lo: u64,

    reg_llbit: bool, // TODO: Enum type
    reg_fcr0: u32,
    reg_fcr31: u32,

    cp0: CP0,

    interconnect: interconnect::Interconnect,
}

impl CPU {
    pub fn new(interconnect: interconnect::Interconnect) -> CPU {
        CPU {
            reg_grp: [0; NUM_GPR],
            reg_fpr: [0; NUM_GPR],

            reg_pc: 0,

            reg_hi: 0,
            reg_lo: 0,

            reg_llbit: false, // TODO: Enum type
            reg_fcr0: 0,
            reg_fcr31: 0,

            cp0: CP0::default(),

            interconnect: interconnect,
        }
    }

    pub fn power_on_reset(&mut self) {
        self.cp0.power_on_reset();

        self.reg_pc = 0xbfc0000;
    }

    pub fn run(&mut self) {
        loop {

        }
    }
}

// TODO: Better name
enum RegConfigEp {
    D,
    DxxDxx,
    RFU,
}

impl Default for RegConfigEp {
    fn default() -> RegConfigEp { RegConfigEp::D }
}

enum RegConfigBe {
    LittleEndian,
    BigEndian,
}

impl Default for RegConfigBe {
    fn default() -> RegConfigBe { RegConfigBe::BigEndian }
}

#[derive(Default)]
struct RegConfig {
    reg_config_ep: RegConfigEp,
    reg_config_be: RegConfigBe,
}

impl RegConfig {
    fn power_on_reset(&mut self) {
        self.reg_config_ep = RegConfigEp::D;
        self.reg_config_be = RegConfigBe::BigEndian;
    }
}

#[derive(Default)]
struct CP0 {
    reg_config: RegConfig,
}

impl CP0 {
    fn power_on_reset(&mut self) {
        self.reg_config.power_on_reset();
    }
}
