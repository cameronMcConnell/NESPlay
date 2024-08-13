use super::bus;
use std::rc::Rc;
use std::cell::RefCell;

type Bus = bus::Bus;

pub struct Ppu {
    bus: Rc<RefCell<Bus>>,
}

impl Ppu {
    pub fn new(bus: Rc<RefCell<Bus>>) -> Self {
        Ppu {
            bus,
        }
    }
}