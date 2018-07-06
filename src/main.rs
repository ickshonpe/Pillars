#![allow(dead_code)]
extern crate sdl2;
extern crate image;
extern crate rand;
mod board;
mod board_analysis;
mod board_partitioning;
mod columns;
mod game;
mod graphics;
mod gravity;
mod input;
mod point2;
mod random;
mod sdl_rendering;

fn main() {
    let pillar_bytes = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/pillar.png"));
    let charset_bytes =include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/charset.png"));
    let image = image::load(std::io::Cursor::new(&pillar_bytes[..]), image::PNG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();


    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("PILLARS!", 800, 600).position_centered().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let texture_creator = canvas.texture_creator();


    'game_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => {
                    break 'game_loop
                },
                _ => ()
            }
        }

        canvas.set_draw_color(sdl2::pixels::Color::RGB(77, 77, 128));
        canvas.clear();
        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        canvas.fill_rect(sdl2::rect::Rect::new(100, 100, 600, 400));
        canvas.present();
        //std::thread::sleep(std::time::Duration::from_millis(1000 / 60));
    }
}
