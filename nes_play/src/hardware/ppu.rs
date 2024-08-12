use super::bus;

type Bus = bus::Bus;

pub struct Ppu<'a> {
    bus: &'a Bus,
}

impl<'a> Ppu<'a> {
    pub fn new(bus: &'a Bus) -> Self {
        Ppu {
            bus,
        }
    }
}