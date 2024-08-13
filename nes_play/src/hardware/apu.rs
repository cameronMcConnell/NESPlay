use super::bus;
use std::rc::Rc;
use std::cell::RefCell;

type Bus = bus::Bus;

pub struct Apu {
    bus: Rc<RefCell<Bus>>,
}

impl Apu {
    pub fn new(bus: Rc<RefCell<Bus>>) -> Self {
        Apu {
            bus
        }
    }
}