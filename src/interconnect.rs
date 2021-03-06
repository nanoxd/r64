use std::fmt;
use byteorder::{BigEndian,ByteOrder};
use mem_map::*;
use rsp::RSP;
use peripheral_interface::PeripheralInterface;
use video_interface::VideoInterface;
use audio_interface::AudioInterface;

const RAM_SIZE: usize = 4 * 1024 * 1024;

pub struct Interconnect {
    pif_rom: Box<[u8]>,
    ai: AudioInterface,
    pi: PeripheralInterface,
    vi: VideoInterface,
    ram: Box<[u16]>,
    rsp: RSP,
}

impl Interconnect {
    pub fn new(pif_rom: Box<[u8]>) -> Interconnect {
        Interconnect {
            rsp: RSP::new(),
            pif_rom: pif_rom,
            ram: vec![0; RAM_SIZE].into_boxed_slice(),
            ai: AudioInterface::default(),
            pi: PeripheralInterface::default(),
            vi: VideoInterface::default(),
        }
    }

    pub fn read_word(&self, addr: u32) -> u32 {
        match map_addr(addr) {
            Addr::PifRom(offset) => BigEndian::read_u32(&self.pif_rom[offset as usize..]),
            Addr::SpImem(offset) => self.rsp.read_imem(offset),
            Addr::SpStatusReg => self.rsp.read_status_reg(),
            Addr::SpDmaBusyReg => self.rsp.read_dma_busy_reg(),
            Addr::PiStatusReg => self.pi.read_status_reg(),
            Addr::ViIntrReg => self.vi.read_intr_reg(),
            Addr::ViHStartReg => self.vi.read_h_start_reg(),
            Addr::ViCurrentReg => self.vi.read_current_reg(),
            Addr::AiDramAddrReg => self.ai.read_dram_addr_reg(),
            Addr::AiLenReg => self.ai.read_len_reg(),
        }
    }

    pub fn write_word(&mut self, addr: u32, value: u32) {
        match map_addr(addr) {
            Addr::PifRom(_) => panic!("Cannot write to PIF ROM"),
            Addr::SpImem(offset) => self.rsp.write_imem(offset, value),
            Addr::SpStatusReg => self.rsp.write_status_reg(value),
            Addr::SpDmaBusyReg => self.rsp.write_dma_busy_reg(value),
            Addr::PiStatusReg => self.pi.write_status_reg(value),
            Addr::ViIntrReg => self.vi.write_intr_reg(value),
            Addr::ViHStartReg => self.vi.write_h_start_reg(value),
            Addr::ViCurrentReg => self.vi.write_current_reg(value),
            Addr::AiDramAddrReg => self.ai.write_dram_addr_reg(value),
            Addr::AiLenReg => self.ai.write_len_reg(value),
        }
    }
}

impl fmt::Debug for Interconnect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TODO: Implement Debug for Interconnect")
    }
}
