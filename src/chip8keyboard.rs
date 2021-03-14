use sdl2::keyboard::Keycode;
use crate::config;
pub struct Chip8keyboard {
    pub keyboard: Vec<bool>,
}

impl Chip8keyboard {
    pub fn new() -> Self {
        Chip8keyboard {
            keyboard: vec![false; config::CHIP8_TOTAL_KEYS as usize],
        } 
    }

    pub fn chip8_keyboard_inbounds(key: u16) {
        assert!(key < config::CHIP8_TOTAL_KEYS);
    }

    pub fn chip8_keyboard_map(map: &Vec<Keycode>, key: Keycode) -> i16 {
        let index = map.iter().position(|&ch| ch == key);
        match index {
            Some(value) => value as i16,
            None => -1,
        }
    }

    pub fn chip8_keyboard_is_down(&self, key: u16) -> bool {
        self.keyboard[key as usize]
    }

    pub fn chip8_keyboard_down(&mut self, key: u16) {
        self.keyboard[key as usize] = true;        
    }

    pub fn chip8_keyboard_up(&mut self, key: u16) {
        self.keyboard[key as usize] = false;
    }
}