use super::bus;

type BUS = bus::Bus;

pub struct Cpu<'a> {
    program_counter: u16,
    stack_pointer: u8,
    register_a: u8,
    register_x: u8,
    register_y: u8,
    processor_status: u8,
    bus: &'a BUS
}

impl<'a> Cpu<'a> {
    pub fn new(bus: &'a BUS) -> Self {
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
}