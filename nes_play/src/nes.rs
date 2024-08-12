use super::hardware;
use lazy_static::lazy_static;

type Cpu<'a> = hardware::cpu::Cpu<'a>;
type Ppu<'a> = hardware::ppu::Ppu<'a>;
type Apu<'a> = hardware::apu::Apu<'a>;
type Bus = hardware::bus::Bus;

pub struct Nes<'a> {
    cpu: Cpu<'a>,
    ppu: Ppu<'a>,
    apu: Apu<'a>,
}

impl<'a> Nes<'a> {
    pub fn new() -> Self {
        Nes {
            cpu: Cpu::new(&NES_BUS),
            ppu: Ppu::new(&NES_BUS),
            apu: Apu::new(&NES_BUS),
        }
    }
}

lazy_static! {
    static ref NES_BUS: Bus = Bus::new();
}