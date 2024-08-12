use lazy_static::lazy_static;
pub struct Opcode {
    code: u8,
    title: &'static str,
    bytes: u8,
    cycles: u8,
    adressing_mode: AddressingModes,
}

enum AddressingModes {
    Implicit,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
    Implied,
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

lazy_static! {
    pub static ref CPU_OP_CODES: Vec<Opcode> = vec![
        Opcode::new(0x69, "ADC", 2, 2, AddressingModes::Immediate),
        Opcode::new(0x65, "ADC", 2, 3, AddressingModes::ZeroPage),
        Opcode::new(0x75, "ADC", 2, 4, AddressingModes::ZeroPageX),
        Opcode::new(0x6D, "ADC", 3, 4, AddressingModes::Absolute),
        Opcode::new(0x7D, "ADC", 3, 4 /*+1 if page crossed*/, AddressingModes::AbsoluteX),
        Opcode::new(0x79, "ADC", 3, 4 /*+1 if page crossed*/, AddressingModes::AbsoluteY),
        Opcode::new(0x61, "ADC", 2, 6, AddressingModes::IndirectX),
        Opcode::new(0x71, "ADC", 2, 5 /*+1 if page crossed*/, AddressingModes::IndirectY),
        
        Opcode::new(0x29, "AND", 2, 2, AddressingModes::Immediate),
        Opcode::new(0x25, "AND", 2, 3, AddressingModes::ZeroPage),
        Opcode::new(0x35, "AND", 2, 4, AddressingModes::ZeroPageX),
        Opcode::new(0x2D, "AND", 3, 4, AddressingModes::Absolute),
        Opcode::new(0x3D, "AND", 3, 4 /*+1 if page crossed*/, AddressingModes::AbsoluteX),
        Opcode::new(0x39, "AND", 3, 4 /*+1 if page crossed*/, AddressingModes::AbsoluteY),
        Opcode::new(0x21, "AND", 2, 6, AddressingModes::IndirectX),
        Opcode::new(0x31, "AND", 2, 5 /*+1 if page crossed*/, AddressingModes::IndirectY),

        Opcode::new(0x0A, "ASL", 1, 2, AddressingModes::Accumulator),
        Opcode::new(0x06, "ASL", 2, 5, AddressingModes::ZeroPage),
        Opcode::new(0x16, "ASL", 2, 6, AddressingModes::ZeroPageX),
        Opcode::new(0x0E, "ASL", 3, 6, AddressingModes::Absolute),
        Opcode::new(0x1E, "ASL", 3, 7, AddressingModes::AbsoluteX),

        Opcode::new(0x90, "BCC", 2, 2 /*+1 if branch succeeds +2 if to a new page*/, AddressingModes::Relative),

        Opcode::new(0xB0, "BCS", 2, 2 /*+1 if branch succeeds +2 if to a new page*/, AddressingModes::Relative),

        Opcode::new(0xF0, "BEQ", 2, 2 /*+1 if branch succeeds +2 if to a new page*/, AddressingModes::Relative),

        Opcode::new(0x24, "BIT", 2, 3, AddressingModes::ZeroPage),
        Opcode::new(0x2c, "BIT", 3, 4, AddressingModes::Absolute),

        Opcode::new(0x30, "BMI", 2, 2 /*+1 if branch succeeds +2 if to a new page*/, AddressingModes::Relative),

        Opcode::new(0xD0, "BNE", 2, 2 /*+1 if branch succeeds +2 if to a new page*/, AddressingModes::Relative),

        Opcode::new(0x10, "BPL", 2, 2 /*+1 if branch succeeds +2 if to a new page*/, AddressingModes::Relative),

        Opcode::new(0x00, "BRK", 1, 7, AddressingModes::Implied),

        Opcode::new(0x50, "BVC", 2, 2 /*+1 if branch succeeds +2 if to a new page*/, AddressingModes::Relative),

        Opcode::new(0x70, "BVS", 2, 2 /*+1 if branch succeeds +2 if to a new page*/, AddressingModes::Relative),

        Opcode::new(0x18, "CLC", 1, 2, AddressingModes::Implied),

        Opcode::new(0xD8, "CLD", 1, 2, AddressingModes::Implied),

        Opcode::new(0x58, "CLI", 1, 2, AddressingModes::Implied),

        Opcode::new(0xB8, "CLV", 1, 2, AddressingModes::Implied),

        Opcode::new(0xC9, "CMP", 2, 2, AddressingModes::Immediate),
        Opcode::new(0xC5, "CMP", 2, 3, AddressingModes::ZeroPage),
        Opcode::new(0xD5, "CMP", 2, 4, AddressingModes::ZeroPageX),
        Opcode::new(0xCD, "CMP", 3, 4, AddressingModes::Absolute),
        Opcode::new(0xDD, "CMP", 3, 4 /*+1 if page crossed*/, AddressingModes::AbsoluteX),
        Opcode::new(0xD9, "CMP", 3, 4 /*+1 if page crossed*/, AddressingModes::AbsoluteY),
        Opcode::new(0xC1, "CMP", 2, 6, AddressingModes::IndirectX),
        Opcode::new(0xD1, "CMP", 2, 5 /*+1 if page crossed*/, AddressingModes::IndirectY),

        Opcode::new(0xE0, "CPX", 2, 2, AddressingModes::Immediate),
        Opcode::new(0xE4, "CPX", 2, 3, AddressingModes::ZeroPage),
        Opcode::new(0xEC, "CPX", 3, 4, AddressingModes::Absolute),

        Opcode::new(0xC0, "CPY", 2, 2, AddressingModes::Immediate),
        Opcode::new(0xC4, "CPY", 2, 3, AddressingModes::ZeroPage),
        Opcode::new(0xCC, "CPY", 3, 4, AddressingModes::Absolute),

        Opcode::new(0xC6, "DEC", 2, 5, AddressingModes::ZeroPage),
        Opcode::new(0xD6, "DEC", 2, 6, AddressingModes::ZeroPageX),
        Opcode::new(0xCE, "DEC", 3, 6, AddressingModes::Absolute),
        Opcode::new(0xDE, "DEC", 3, 7, AddressingModes::AbsoluteX),

        Opcode::new(0xCA, "DEX", 1, 2, AddressingModes::Implied),

        Opcode::new(0x88, "DEY", 1, 2, AddressingModes::Implied),

        Opcode::new(0x49, "EOR", 2, 2, AddressingModes::Immediate),
        Opcode::new(0x45, "EOR", 2, 3, AddressingModes::ZeroPage),
        Opcode::new(0x55, "EOR", 2, 4, AddressingModes::ZeroPageX),
        Opcode::new(0x4D, "EOR", 3, 4, AddressingModes::Absolute),
        Opcode::new(0x5D, "EOR", 3, 4 /*+1 if page crossed*/, AddressingModes::AbsoluteX),
        Opcode::new(0x59, "EOR", 3, 4 /*+1 if page crossed*/, AddressingModes::AbsoluteY),
        Opcode::new(0x41, "EOR", 2, 6, AddressingModes::IndirectX),
        Opcode::new(0x51, "EOR", 2, 5 /*+1 if page crossed*/, AddressingModes::IndirectY),

        Opcode::new(0xE6, "INC", 2, 5, AddressingModes::ZeroPage),
        Opcode::new(0xF6, "INC", 2, 6, AddressingModes::ZeroPageX),
        Opcode::new(0xEE, "INC", 3, 6, AddressingModes::Absolute),
        Opcode::new(0xFE, "INC", 3, 7, AddressingModes::AbsoluteX),

        Opcode::new(0xE8, "INX", 1, 2, AddressingModes::Implied),

        Opcode::new(0xC8, "INY", 1, 2, AddressingModes::Implied),
    ];
}