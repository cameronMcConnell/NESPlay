use super::bus;
use super::opcode;

type Bus = bus::Bus;

pub struct Cpu<'a> {
    program_counter: u16,
    stack_pointer: u8,
    register_a: u8,
    register_x: u8,
    register_y: u8,
    processor_status: u8,
    bus: &'a Bus
}

impl<'a> Cpu<'a> {
    pub fn new(bus: &'a Bus) -> Self {
        Cpu {
            program_counter: 0,
            stack_pointer: 0xFF,
            register_a: 0,
            register_x: 0,
            register_y: 0,
            processor_status: 0,
            bus,
        }
    }

    pub fn inc_program_counter(&mut self) {
        self.program_counter += 1;
    }

    pub fn set_program_counter(&mut self, address: u16) {
        self.program_counter = address;
    }

    pub fn dec_stack_pointer(&mut self) {
        self.stack_pointer -= 1;
    }

    pub fn execute_opcode(&mut self) {
        let opcode = &opcode::CPU_OP_CODES[self.program_counter as usize];
        self.inc_program_counter();
    }
}