use super::hardware;
use lazy_static::lazy_static;

type CPU<'a> = hardware::cpu::Cpu<'a>;
type PPU<'a> = hardware::ppu::Ppu<'a>;
type APU<'a> = hardware::apu::Apu<'a>;
type BUS = hardware::bus::Bus;

pub struct Nes<'a> {
    cpu: CPU<'a>,
    ppu: PPU<'a>,
    apu: APU<'a>,
}

impl<'a> Nes<'a> {
    pub fn new() -> Self {
        Nes {
            cpu: CPU::new(&NES_BUS),
            ppu: PPU::new(&NES_BUS),
            apu: APU::new(&NES_BUS),
        }
    }
}

lazy_static! {
    static ref NES_BUS: BUS = BUS::new();
}