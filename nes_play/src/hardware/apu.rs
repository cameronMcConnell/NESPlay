use super::bus;

type Bus = bus::Bus;

pub struct Apu<'a> {
    bus: &'a Bus
}

impl<'a> Apu<'a> {
    pub fn new(bus: &'a Bus) -> Self {
        Apu {
            bus
        }
    }
}