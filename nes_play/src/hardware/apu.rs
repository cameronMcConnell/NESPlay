use super::bus;

type BUS = bus::Bus;

pub struct Apu<'a> {
    bus: &'a BUS
}

impl<'a> Apu<'a> {
    pub fn new(bus: &'a BUS) -> Self {
        Apu {
            bus
        }
    }
}