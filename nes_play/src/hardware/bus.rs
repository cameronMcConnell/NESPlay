const RAM: u16 = 0x0000;
const RAM_MIRRORS_END: u16 = 0x1FFF;
const PPU_REGISTERS: u16 = 0x2000;
const PPU_REGISTERS_MIRRORS_END: u16 = 0x3FFF;

pub struct Bus {
    vram: [u8; 2048],
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            vram: [0; 2048],
        }
    }

    pub fn reset(&mut self) {
        self.vram = [0; 2048];
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            RAM ..= RAM_MIRRORS_END => {
                let mirror_address = address & 0b00000111_11111111;
                self.vram[mirror_address as usize]
            },
            PPU_REGISTERS ..= PPU_REGISTERS_MIRRORS_END => {
                let mirror_address = address & 0b00100000_00000111;;
                self.vram[mirror_address as usize]
            },
            _ => {
                println!("Ingoring read access to vram at address {}.", address);
                0
            },
        }
    }

    pub fn write(&mut self, address: u16, data: u8) {
        match address {
            RAM ..= RAM_MIRRORS_END => {
                let mirror_address = address & 0b00000111_11111111;
                self.vram[mirror_address as usize] = data;
            },
            PPU_REGISTERS ..= PPU_REGISTERS_MIRRORS_END => {
                let mirror_address = address & 0b00100000_00000111;;
                self.vram[mirror_address as usize] = data;
            },
            _ => {
                println!("Ingoring write access to vram at address {}.", address);
            },
        }
    }

    pub fn load_program(&mut self, path: &str) {

    }
}