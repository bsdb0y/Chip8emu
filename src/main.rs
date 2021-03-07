use sdl2::{pixels::Color, rect::Rect};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod config;

struct Chip8 {
    pub mem: Chip8mem,
    pub regs: Chip8regs,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            mem: Chip8mem::new(),
            regs: Chip8regs::new(),
        }
    }
}

struct Chip8mem {
    pub memory: Vec<u8>,
 }
 
 impl Chip8mem {
    pub fn new() -> Self {
        Chip8mem {
            memory: vec![0u8;config::CHIP8_MEMORY_SIZE as usize],
        }
    }
 }

 struct Chip8regs {
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

 fn chip8_mem_set(memory: &mut Chip8mem, index: i32, val: u8) -> Option<u8> {
    if index as usize > config::CHIP8_MEMORY_SIZE as usize {
        return None;
    }
    memory.memory[index as usize] = val;
    Some(val)
}
 
 fn chip8_mem_get(memory: &mut Chip8mem, index: i32) -> Result<u8, &'static str> {
    match memory.memory.get(index as usize) {
        Some(value) => Ok(*value),
        None => Err("out of bound error")
    }
 }

 fn main() -> Result<(), String> {
    let mut chip8 = Chip8::new();
    
    chip8.regs.v[0x0f] = 55; //implemented register

    if chip8_mem_set(&mut chip8.mem, 50, b'A') == None {
        println!("chip8_mem_set: Out of bound access");
        std::process::exit(-1);
    }

    let result = chip8_mem_get(&mut chip8.mem, 50);
    if result.is_err() {
        println!("chip8_mem_get: Out of bound access");
        std::process::exit(-1);
    }
    println!("{:x?}", result.unwrap());
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window(config::EMULATOR_WINDOW_TITLE,
         config::CHIP8_WIDTH * config::CHIP8_WINDOW_MULTIPLIER,
         config::CHIP8_HEIGHT * config::CHIP8_WINDOW_MULTIPLIER)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    let rect = Rect::new(0, 0, 40, 40);
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    canvas.set_draw_color(Color::RGB(255, 255, 0));
    canvas.fill_rect(rect)?;
    /*uncommenting this line won't work as it will fill
     *the color to the canvas but we need only in rectangle
     */
    // canvas.clear();
    canvas.present(); /*new frame rendering to the window */

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            if let Event::Quit {..} |
                            Event::KeyDown { keycode: Some(Keycode::Escape), .. } = event {
                break 'running;
            }
        }
    }
    Ok(())
}
