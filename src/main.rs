#![allow(dead_code)]
extern crate sdl2;
extern crate image;
extern crate rand;
#[macro_use] extern crate maplit;
extern crate gl;

mod board;
mod board_analysis;
mod board_partitioning;
mod columns;
mod events;
mod game;
mod gl_rendering;
mod graphics;
mod gravity;
mod input;
mod point2;
mod random;
mod sdl_rendering;
mod shaders;

use point2::*;
use board::*;


fn main() {


//    let pillar_bytes = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/pillar.png"));
//    let charset_bytes =include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/charset.png"));
//    let image = image::load(std::io::Cursor::new(&pillar_bytes[..]), image::PNG).unwrap().to_rgba();
//    let image_dimensions = image.dimensions();
    let cell_size = [32, 32];
    let cell_padding = [2, 2];
    let key_bindings: std::collections::HashMap<sdl2::keyboard::Keycode, input::Buttons> = hashmap!{
        sdl2::keyboard::Keycode::Left => input::Buttons::Left,
        sdl2::keyboard::Keycode::Right => input::Buttons::Right,
        sdl2::keyboard::Keycode::Down => input::Buttons::Down,
        sdl2::keyboard::Keycode::Z => input::Buttons::CycleUp,
        sdl2::keyboard::Keycode::X => input::Buttons::CycleDown,
        sdl2::keyboard::Keycode::Escape => input::Buttons::Quit
    };

    let vertices: Vec<f32> = vec![
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0, 0.5, 0.0
    ];



    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window =
        video_subsystem
            .window("PILLARS!", 600, 600)
            .opengl()
            .position_centered()
            .build()
            .unwrap();
    let gl_context = window.gl_create_context().unwrap();
    let _ = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);



    let mut event_pump = sdl_context.event_pump().unwrap();
    //let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    let mut game_data = game::GameData::default();
    let mut input_state = input::InputState::default();
    let mut ticks = 0;
    let mut vbo: gl::types::GLuint = 0;
    let mut vao: gl::types::GLuint = 0;
    unsafe {
        use gl::types::*;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (3 * std::mem::size_of::<f32>()) as gl::types::GLsizei,
            std::ptr::null()
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
        gl::Viewport(0, 0, 600, 600);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let shaders= [
        gl_rendering::Shader::from_str(shaders::VERTEX_SHADER, gl::VERTEX_SHADER).unwrap(),
        gl_rendering::Shader::from_str(shaders::FRAGMENT_SHADER, gl::FRAGMENT_SHADER).unwrap()
    ];
    let shader_program = gl_rendering::link_program(&shaders).unwrap();
    gl_rendering::use_program(&shader_program);
    
    'game_loop: loop {
//        ticks += 1;
//        if game_data.game_over {
//            let high_score = if game_data.score > game_data.high_score { game_data.score } else { game_data.high_score };
//            game_data = game::GameData::default();
//            game_data.high_score = high_score;
//        }
//
            input_state.store_current();
            for event in event_pump.poll_iter() {
                events::process_sdl_event(&event, &mut input_state, &key_bindings);
            }
            if input_state.down(input::Buttons::Quit) {
                break 'game_loop;
            }
//
//        game::update_game(&mut game_data, &input_state, 1.0 / 60.0 );
//
//        canvas.set_draw_color(sdl2::pixels::Color::RGB(77, 77, 128));
//        canvas.clear();
//        canvas.set_draw_color(sdl2::pixels::Color::RGB(210, 195, 195));
//        let _ = canvas.fill_rect(sdl2::rect::Rect::new(100, 100 + 32, (game_data.board.width() as u32) * (cell_size[0] + cell_padding[0]) + cell_padding[0], 400));
//
//        sdl_rendering::draw_board(&mut canvas, &game_data.board, game_data.current_column, [100, 100], cell_size, cell_padding);
//        canvas.present();
        unsafe {

            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);


        }

        window.gl_swap_window();

        std::thread::sleep(std::time::Duration::from_millis(1000 / 60));
    }


}

