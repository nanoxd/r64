use super::opcode;

pub struct Instruction(u32);

impl Instruction {
    pub fn opcode(&self) -> Opcode {
        self.into()
    }

    pub fn rs(&self) -> u32 {
        (self.0 >> 21) & 0b11111
    }

    pub fn rt(&self) -> u32 {
        (self.0 >> 16) & 0b11111
    }

    #[inline(always)]
    pub fn imm(&self) -> u32 {
        self.0 & 0xffff
    }

    pub fn offset(&self) -> u32 {
        self.imm()
    }
}
