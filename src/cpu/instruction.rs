pub struct Instruction(u32);

impl Instruction {
    pub fn rs(&self) -> u32 {
        (self >> 21) & 0b11111
    }

    pub fn rt(&self) -> u32 {
        (instruction >> 16) & 0b11111
    }

    pub fn imm(&self) -> u32 {
        instruction & 0xffff
    }

    pub fn offset(&self) -> u32 {
        self.imm()
    }
}
