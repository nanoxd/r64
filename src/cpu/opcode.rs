enum_from_primitive! {
    #[derive(Debug)]
    pub enum Opcode {
        Special = 0b000000,
        Addi    = 0b001000,
        Addiu   = 0b001001,

        Andi    = 0b001100,
        Ori     = 0b001101,

        Lui     = 0b001111,
        Mtc0    = 0b010000,

        Beql    = 0b010100,
        Bne     = 0b000101,
        Bnel    = 0b010101,

        Lw      = 0b100011,
        Sw      = 0b101011,
    }
}
