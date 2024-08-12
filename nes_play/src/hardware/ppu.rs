use super::bus;

type BUS = bus::Bus;

pub struct Ppu<'a> {
    bus: &'a BUS,
}

impl<'a> Ppu<'a> {
    pub fn new(bus: &'a BUS) -> Self {
        Ppu {
            bus,
        }
    }
}