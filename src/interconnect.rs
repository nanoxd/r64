use std::fmt;
use byteorder::{BigEndian,ByteOrder};
use mem_map::*;
use rsp::RSP;

const RAM_SIZE: usize = 4 * 1024 * 1024;

pub struct Interconnect {
    pif_rom: Box<[u8]>,
    ram: Box<[u16]>,
    rsp: RSP,
}

impl Interconnect {
    pub fn new(pif_rom: Box<[u8]>) -> Interconnect {
        Interconnect {
            rsp: RSP::default(),
            pif_rom: pif_rom,
            ram: vec![0; RAM_SIZE].into_boxed_slice()
        }
    }

    pub fn read_word(&self, addr: u32) -> u32 {
        match addr {
            PIF_ROM_START ... PIF_ROM_END => {
                let rel_addr = addr - PIF_ROM_START;

                BigEndian::read_u32(&self.pif_rom[rel_addr as usize..])
            },
            SP_STATUS_REG => self.rsp.read_status_reg(),
            _ => panic!("Unrecognized physical address {:#x}", addr)
        }
    }

        if addr >= PIF_ROM_START && addr < PIF_ROM_END {
            let rel_addr = addr - PIF_ROM_START;

            BigEndian::read_u32(&self.pif_rom[rel_addr as usize..])
        } else {
            match addr {
                SP_STATUS_REG => {
                    self.rsp.read_status_reg()
                },
                _ => panic!("Unrecognized physical address {:#x}", addr)
            }
        }
    }
}

impl fmt::Debug for Interconnect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TODO: Implement Debug for Interconnect")
    }
}
