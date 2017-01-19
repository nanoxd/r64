use super::opcode;

pub struct Instruction(u32);

impl Instruction {
    #[inline(always)]
    pub fn opcode(&self) -> Opcode {
        self.into()
    }

    #[inline(always)]
    pub fn rs(&self) -> u32 {
        (self.0 >> 21) & 0b11111
    }

    #[inline(always)]
    pub fn rt(&self) -> u32 {
        (self.0 >> 16) & 0b11111
    }

    #[inline(always)]
    pub fn imm(&self) -> u32 {
        self.0 & 0xffff
    }

    #[inline(always)]
    pub fn offset(&self) -> u32 {
        self.imm()
    }
}
