use sdl2::{pixels::Color, rect::Rect};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod config;

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

 fn chip8_mem_set(memory: &mut Chip8mem, index: i32, val: u8) {
    memory.memory[index as usize] = val;
 }
 
 fn chip8_mem_get(memory: &mut Chip8mem, index: i32) -> u8{
    memory.memory[index as usize]
 }

 fn main() -> Result<(), String> {
    let chip8_mem = &mut Chip8mem::new();
    chip8_mem_set(chip8_mem, 50, b'A');
    println!("{:x?}", chip8_mem_get(chip8_mem, 50));

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
