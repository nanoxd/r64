use ::interconnect;
use super::cp0::cp0::CP0;

const NUM_GPR: usize = 32;

#[derive(Debug)]
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

        self.reg_pc = 0xffff_ffff_bfc0_0000;
    }

    pub fn run(&mut self) {
        loop {
            self.run_instruction();
        }
    }

    pub fn run_instruction(&mut self) {
        let instruction = self.read_word(self.reg_pc);

        let opcode = (instruction >> 26) & 0b111111;
        let rs = (instruction >> 21) & 0b11111;
        let rt = (instruction >> 16) & 0b11111;
        let imm = instruction & 0xffff;

        match opcode {
            0b001101 => { // ORI
                let res = self.read_reg_gpr(rs as usize) | (imm as u64);
                self.write_reg_gpr(rt as usize, res);
            },
            0b001111 => { // LUI
                // TODO
                self.write_reg_gpr(rt as usize, (imm << 16) as u64);
            },
            0b010000 => { // MTC0
                let rd = (instruction >> 11) & 0b11111;
                let data = self.read_reg_gpr(rt as usize);
                self.cp0.write_reg(rd as u32, data);
            },
            _ => {
                panic!("Unrecognized opcode: {:#x}", instruction)
            }
        }

        self.reg_pc += 4;
    }

    fn read_reg_gpr(&self, index: usize) -> u64 {
        match index {
            0 => 0,
            _ => self.reg_grp[index]
        }
    }

    fn write_reg_gpr(&mut self, index: usize, value: u64) {
        if index != 0 {
            self.reg_grp[index] = value;
        }
    }

    fn read_word(&self, virt_addr: u64) -> u32 {
        let phys_addr = self.virtual_addr_to_phys_addr(virt_addr);

        self.interconnect.read_word(phys_addr as u32)
    }

    fn virtual_addr_to_phys_addr(&self, virt_addr: u64) -> u64 {
        // See Table 5-3 of Section 5.3 of VR4300 User's Manual
        let addr_bit_values = (virt_addr >> 29) & 0b111;

        if addr_bit_values == 0b101 { // kseg1
            virt_addr - 0xffff_ffff_a000_0000
        } else {
            panic!("Unrecognized virtual address: {:#x}", virt_addr);
        }
    }
}
