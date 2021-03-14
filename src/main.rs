use chip8memory::Chip8mem;
use sdl2::{pixels::Color, rect::Rect};
use sdl2::event::Event;

mod config;
mod chip8memory;
mod chip8keyboard;

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
 

fn main() -> Result<(), String> {

    //TODO: implement the keyboard and keycode
    let nums = vec![
        sdl2::keyboard::Keycode::Num0,
        sdl2::keyboard::Keycode::Num1,
        sdl2::keyboard::Keycode::Num2,
        sdl2::keyboard::Keycode::Num3,
        sdl2::keyboard::Keycode::Num4,
        sdl2::keyboard::Keycode::Num5,
        sdl2::keyboard::Keycode::Num6,
        sdl2::keyboard::Keycode::Num7,
        sdl2::keyboard::Keycode::Num8,
        sdl2::keyboard::Keycode::Num9,
        sdl2::keyboard::Keycode::A,
        sdl2::keyboard::Keycode::B,
        sdl2::keyboard::Keycode::C,
        sdl2::keyboard::Keycode::D,
        sdl2::keyboard::Keycode::E,
        sdl2::keyboard::Keycode::F
        ];

    let mut chip8 = Chip8::new();

    println!("[+] Implemented registers");
    chip8.regs.v[0x0f] = 55;

    println!("[+] Implemented memory subsystem");
    if Chip8mem::chip8_mem_set(&mut chip8.mem, 50, b'A') == None {
        println!("chip8_mem_set: Out of bound access");
        std::process::exit(-1);
    }

    let result = Chip8mem::chip8_mem_get(&mut chip8.mem, 50);
    if result.is_err() {
        println!("chip8_mem_get: Out of bound access");
        std::process::exit(-1);
    }
    println!("{:x?}", result.unwrap());
    
    println!("[+] Implemented stack subsystem");
    chip8memory::Chip8stack::chip8_stack_push(&mut chip8, 0xff);
    chip8memory::Chip8stack::chip8_stack_push(&mut chip8, 0xaa);
    println!("{:x?}", chip8memory::Chip8stack::chip8_stack_pop(&mut chip8));
    println!("{:x?}", chip8memory::Chip8stack::chip8_stack_pop(&mut chip8));
    
    println!("[+] Implemendted keyboard");
    chip8keyboard::Chip8keyboard::chip8_keyboard_down(&mut chip8.keyboard, 0x0f);
    let is_down: bool = chip8keyboard::Chip8keyboard::chip8_keyboard_is_down(&chip8.keyboard, 0x0f);
    println!("{:?}", is_down);

    chip8keyboard::Chip8keyboard::chip8_keyboard_up(&mut chip8.keyboard, 0x0f);
    let is_down: bool = chip8keyboard::Chip8keyboard::chip8_keyboard_is_down(&chip8.keyboard, 0x0f);
    println!("{:?}", is_down);

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
            match event {
                Event::Quit {..} => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(key), ..} => {
                    let vkey = chip8keyboard::Chip8keyboard::chip8_keyboard_map(&nums, key);
                    if vkey != -1 {
                        println!("Virtual Key Down: {}", vkey);
                        chip8keyboard::Chip8keyboard::chip8_keyboard_down(&mut chip8.keyboard, vkey as u16);
                    }
                },
                Event::KeyUp { keycode: Some(key), ..} => {
                    let vkey = chip8keyboard::Chip8keyboard::chip8_keyboard_map(&nums, key);
                    if vkey != -1 {
                        println!("Virtual Key Up: {}", vkey);
                        chip8keyboard::Chip8keyboard::chip8_keyboard_up(&mut chip8.keyboard, vkey as u16);
                    }
                },
                _ => {}
            }
        }
    }
    Ok(())
}
