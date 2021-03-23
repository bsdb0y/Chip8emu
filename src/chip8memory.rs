use crate::chip8;
use crate::config;
pub struct Chip8mem {
    pub memory: Vec<u8>,
 }
 
impl Chip8mem {
    pub fn new() -> Self {
        Chip8mem {
            memory: vec![0u8;config::CHIP8_MEMORY_SIZE as usize],
        }
    }

    pub fn chip8_mem_set(&mut self, index: i32, val: u8) -> Option<u8> {
        if index as usize > config::CHIP8_MEMORY_SIZE as usize {
            return None;
        }
        self.memory[index as usize] = val;
        Some(val)
    }
    
     
     pub fn chip8_mem_get(&mut self, index: i32) -> Result<u8, &'static str> {
        match self.memory.get(index as usize) {
            Some(value) => Ok(*value),
            None => Err("out of bound error")
        }
     }
 }

 pub struct Chip8regs {
    pub v: Vec<u8>,
    pub i: u16,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub pc: u16,
    pub sp: u8,
 }

 impl Chip8regs {
     pub fn new() -> Self {
         Chip8regs {
             v: vec![0u8;config::CHIP8_TOTAL_DATA_REGS as usize],
             i: 0u16,
             delay_timer: 0u8,
             sound_timer: 0u8,
             pc: 0u16,
             sp: 0u8,
         }
     }
 }

pub struct Chip8stack {
    pub stack: Vec<u16>,
}

impl Chip8stack {
    pub fn new() -> Self {
        Chip8stack {
            stack: vec![0u16; config::CHIP8_TOTAL_STACK_DEPTH as usize],
        }
    }

    pub fn chip8_stack_inbounds(chip8: &mut chip8::Chip8) {
        assert!((chip8.regs.sp as u16) < config::CHIP8_TOTAL_STACK_DEPTH);
    }

    pub fn chip8_stack_push(chip8: &mut chip8::Chip8, val: u16) {
        self::Chip8stack::chip8_stack_inbounds(chip8);
        chip8.stac.stack[chip8.regs.sp as usize] = val;
        chip8.regs.sp += 1;
    }

    pub fn chip8_stack_pop(chip8: &mut chip8::Chip8) -> u16 {
        chip8.regs.sp -= 1;
        self::Chip8stack::chip8_stack_inbounds(chip8);
        chip8.stac.stack[chip8.regs.sp as usize]
    }
}