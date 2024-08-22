pub struct Bus {
    vram: [u8; 2048],
    ppu_clock_cycle: u64,
    apu_clock_cycle: u64,
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            vram: [0; 2048],
            ppu_clock_cycle: 0,
            apu_clock_cycle: 0,
        }
    }

    pub fn reset(&mut self) {
        self.vram = [0; 2048];
        self.ppu_clock_cycle = 0;
        self.apu_clock_cycle = 0;
    }
    
    pub fn inc_ppu_clock_cycle(&mut self) {
        self.ppu_clock_cycle += 1;
    }
    
    pub fn inc_apu_clock_cycle(&mut self) {
        self.apu_clock_cycle += 1;
    }

    pub fn read(&self, address: u16) -> u8 {
        self.vram[address as usize]
    }

    // This needs mirroring
    pub fn write(&mut self, address: u16, data: u8) {
        
    }

    pub fn load_program(&mut self, path: &str) {

    }
}