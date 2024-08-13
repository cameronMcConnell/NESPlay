use super::hardware;
use std::rc::Rc;
use std::cell::RefCell;

type Cpu = hardware::cpu::Cpu;
type Ppu = hardware::ppu::Ppu;
type Apu = hardware::apu::Apu;
type Bus = hardware::bus::Bus;

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