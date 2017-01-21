use ::cpu::CPU;
use ::interconnect::Interconnect;

#[derive(Debug)]
pub struct N64 {
    cpu: CPU,
}

impl N64 {
    pub fn new(pif_rom: Box<[u8]>) -> N64 {
        let interconnect = Interconnect::new(pif_rom);
        let cpu = CPU::new(interconnect);

        N64 {
            cpu: cpu,
        }
    }
    
    pub fn run(&mut self) {
        self.cpu.run()
    }

    pub fn run_instruction(&mut self) {
        self.cpu.run_instruction();
    }
}
