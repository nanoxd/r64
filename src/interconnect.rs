use std::fmt;
use byteorder::{BigEndian,ByteOrder};
use mem_map::*;
use rsp::RSP;
use peripheral_interface::PeripheralInterface;

const RAM_SIZE: usize = 4 * 1024 * 1024;

pub struct Interconnect {
    pif_rom: Box<[u8]>,
    pi: PeripheralInterface,
    ram: Box<[u16]>,
    rsp: RSP,
}

impl Interconnect {
    pub fn new(pif_rom: Box<[u8]>) -> Interconnect {
        Interconnect {
            rsp: RSP::new(),
            pif_rom: pif_rom,
            ram: vec![0; RAM_SIZE].into_boxed_slice(),
            pi: PeripheralInterface::default(),
        }
    }

    pub fn read_word(&self, addr: u32) -> u32 {
        match map_addr(addr) {
            Addr::PifRom(offset) => BigEndian::read_u32(&self.pif_rom[offset as usize..]),
            Addr::SpStatusReg => self.rsp.read_status_reg(),
            Addr::SpDmaBusyReg => self.rsp.read_dma_busy_reg(),
            Addr::PiStatusReg => self.pi.read_status_reg(),
        }
    }

    pub fn write_word(&mut self, addr: u32, value: u32) {
        match map_addr(addr) {
            Addr::PifRom(_) => panic!("Cannot write to PIF ROM"),
            Addr::SpStatusReg => self.rsp.write_status_reg(value),
            Addr::SpDmaBusyReg => self.rsp.write_dma_busy_reg(value),
            Addr::PiStatusReg => self.pi.write_status_reg(value),
        }
    }
}

impl fmt::Debug for Interconnect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TODO: Implement Debug for Interconnect")
    }
}
