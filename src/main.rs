use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

const NUM_GPR: usize = 32;

struct CPU {
    reg_grp: [u64; NUM_GPR],
    reg_fpr: [u64; NUM_GPR],

    reg_pc: u64,

    reg_hi: u64,
    reg_lo: u64,

    reg_llbit: bool, // TODO: Enum type
    reg_fcr0: u32,
    reg_fcr31: u32,

    cp0: CP0,
}

impl CPU {
    fn new() -> CPU {
        CPU { }
    }

struct CP0 {
    reg_index: u64,
    reg_random: u64,
    reg_entry_lo0: u64,
    reg_entry_lo1: u64,
    reg_context: u64,
    reg_page_mask: u64,
    reg_wired: u64,
    reg_bad_v_addr: u64,
    reg_count: u64,
    reg_entry_hi: u64,
    reg_compare: u64,
    reg_status: u64,
    reg_cause: u64,
    reg_epc: u64,
    reg_pr_id: u64,
    reg_config: u64,
    reg_ll_addr: u64,
    reg_watch_lo: u64,
    reg_watch_hi: u64,
    reg_xcontext: u64,
    reg_tag_lo: u64,
    reg_tag_hi: u64,
    reg_error_epc: u64,
}

impl CP0 {
    fn new() -> CP0 {
        unimplemented!()
    }
}

fn main() {
    let pif_file_name = env::args().nth(1).unwrap();
    let rom_file_name = env::args().nth(2).unwrap();

    let pif = read_bin(pif_file_name);
    let rom = read_bin(rom_file_name);

    let mut cpu = CPU::new();
}

fn read_bin<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file = File::open(path.as_ref()).unwrap();
    let mut file_buffer = Vec::new();
    file.read_to_end(&mut file_buffer).unwrap();

    file_buffer
}
