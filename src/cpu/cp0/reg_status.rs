#[derive(Debug, Default)]
pub struct RegStatus {
    coprocesser_usability: [bool; 4], // CU
    low_power_enable: bool, // RP
    additional_fp_regs_enable: bool, // FR
    reverse_endian_enable: bool, // RE
    diagnostic_status: DiagnosticStatus, //DS
    interrupt_mask: InterruptMask, //IM
    kernel_mode_64bit_addressing_enable: bool, // KX
    supervisor_mode_64bit_addressing_enable: bool, // SX
    user_mode_64bit_addressing_enable: bool, // UX
    mode: Mode, // KSU
    error_level_enable: bool,
    exception_level_enable: bool,
    interrupts_enabled: bool,
}

#[derive(Debug, Default)]
struct DiagnosticStatus {
    instruction_trace_support: bool,
    tlb_general_exception_vector_location: TLBGeneralExceptionVectorLocation,
    tlb_shutdown: bool,
    soft_reset_or_nmi_occurred: bool,
    condition_bit: bool,
}

#[derive(Debug)]
enum TLBGeneralExceptionVectorLocation {
    Normal,
    Bootstrap
}

impl Default for TLBGeneralExceptionVectorLocation {
    fn default() -> TLBGeneralExceptionVectorLocation {
        TLBGeneralExceptionVectorLocation::Normal
    }
}

#[derive(Debug, Default)]
struct InterruptMask {
    timer_interrupt: bool,
    external_interrupt_write_req: [bool; 5],
    software_interrupt_cause_reg: [bool; 2],
}

#[derive(Debug)]
enum Mode {
    Kernel,
    Supervisor,
    User,
}

impl Default for Mode {
    fn default() -> Mode {
        Mode::Kernel
    }
}
