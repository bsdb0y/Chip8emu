use crate::chip8memory;
use crate::chip8keyboard;
pub struct Chip8 {
    pub mem: chip8memory::Chip8mem,
    pub stac: chip8memory::Chip8stack,
    pub regs: chip8memory::Chip8regs,
    pub keyboard: chip8keyboard::Chip8keyboard,

}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            mem: chip8memory::Chip8mem::new(),
            stac: chip8memory::Chip8stack::new(),
            regs: chip8memory::Chip8regs::new(),
            keyboard: chip8keyboard::Chip8keyboard::new(),
        }
    }
}