use ::cpu::CPU;
use ::interconnect::Interconnect;

#[derive(Debug)]
pub struct N64 {
    cpu: CPU,
}

impl N64 {
    pub fn new(pif_rom: Vec<u8>) -> N64 {
        let interconnect = Interconnect::new(pif_rom);
        let cpu = CPU::new(interconnect);

        N64 {
            cpu: cpu,
        }
    }

    pub fn power_on_reset(&mut self) {
        self.cpu.power_on_reset();
    }

    pub fn run(&mut self) {
        self.cpu.run()
    }

    pub fn run_instruction(&mut self) {
        self.cpu.run_instruction();
    }
}
