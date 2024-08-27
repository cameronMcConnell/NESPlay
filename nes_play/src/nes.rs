use super::hardware::cpu::Cpu;
use super::hardware::ppu::Ppu;
use super::hardware::apu::Apu;
use super::hardware::bus::Bus;

use std::rc::Rc;
use std::cell::RefCell;

pub struct Nes {
    cpu: Cpu,
    ppu: Ppu,
    apu: Apu,
    bus: Rc<RefCell<Bus>>,
}

impl Nes {
    pub fn new() -> Self {
        let bus = Rc::new(RefCell::new(Bus::new()));

        Nes {
            cpu: Cpu::new(bus.clone()),
            ppu: Ppu::new(bus.clone()),
            apu: Apu::new(bus.clone()),
            bus,
        }
    }
}