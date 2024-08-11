pub struct Cpu {
    program_counter: u16,
    stack_pointer: u8,
    register_a: u8,
    register_x: u8,
    register_y: u8,
    processor_status: u8,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            program_counter: 0,
            stack_pointer: 0xFF,
            register_a: 0,
            register_x: 0,
            register_y: 0,
            processor_status: 0,
        }
    }
}

struct Opcode {
    code: u8,
    title: &'static str,
    bytes: u8,
    cycles: u8,
    adressing_mode: AddressingModes,
}

enum AddressingModes {
    Absolute,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    AbsoluteX,
    AbsoluteY,
    Immediate,
    Relative,
    Implicit,
    Indirect,
    IndexedIndirect,
    IndirectIndexed,
}

impl Opcode {
    fn new(code: u8, title: &'static str, bytes: u8, cycles: u8, adressing_mode: AddressingModes) -> Self {
        Opcode {
            code,
            title,
            bytes,
            cycles,
            adressing_mode,
        }
    }
}