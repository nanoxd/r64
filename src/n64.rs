use ::cpu::CPU;

#[derive(Default)]
pub struct N64 {
    cpu: CPU,
}

impl N64 {
    pub fn power_on_reset(&mut self) {
        self.cpu.power_on_reset();
    }

    pub fn run(&mut self) {
        self.cpu.run()
    }
}
