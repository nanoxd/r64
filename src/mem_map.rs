const PIF_ROM_START: u32 = 0x1fc0_0000;
const PIF_ROM_LENGTH: u32 = 0x0000_07c0;
const PIF_ROM_END: u32 = PIF_ROM_START + PIF_ROM_LENGTH - 1;

const SP_BASE_REG: u32 = 0x0404_0000;
const SP_STATUS_REG: u32 = 0x0404_0010;
const SP_DMA_BUSY_REG: u32 = 0x0404_0018;

const VI_BASE_REG: u32 = 0x0440_0000;
const VI_INTR_REG: u32 = 0x0440_000c;
const VI_H_START_REG: u32 = 0x0440_0024;
const VI_CURRENT_REG: u32 = 0x0440_0010;

const AI_BASE_REG: u32 = 0x0450_0000;
const AI_DRAM_ADDR_REG: u32 = AI_BASE_REG;
const AI_LEN_REG: u32 = 0x0450_0004;

const PI_BASE_REG: u32 = 0x0460_0000;
const PI_STATUS_REG: u32 = 0x0460_0010;

pub enum Addr {
    PiStatusReg,
    PifRom(u32),

    ViIntrReg,
    ViHStartReg,
    ViCurrentReg,

    SpStatusReg,
    SpDmaBusyReg,

    AiDramAddrReg,
    AiLenReg,
}

pub fn map_addr(addr: u32) -> Addr {
    match addr {
        PI_STATUS_REG => Addr::PiStatusReg,

        PIF_ROM_START ... PIF_ROM_END => Addr::PifRom(addr - PIF_ROM_START),

        SP_STATUS_REG => Addr::SpStatusReg,

        SP_DMA_BUSY_REG => Addr::SpDmaBusyReg,

        VI_INTR_REG => Addr::ViIntrReg,
        VI_H_START_REG => Addr::ViHStartReg,
        VI_CURRENT_REG => Addr::ViCurrentReg,

        AI_DRAM_ADDR_REG => Addr::AiDramAddrReg,
        AI_LEN_REG => Addr::AiLenReg,

        _ => panic!("Unrecognized physical address {:#x}", addr)
    }
}
