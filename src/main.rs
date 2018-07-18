#![allow(dead_code)]
extern crate sdl2;
extern crate image;
extern crate rand;
#[macro_use] extern crate maplit;
extern crate time;

mod board;
mod board_analysis;
mod board_partitioning;
mod charset;
mod columns;
mod events;
mod game;
mod gl;
mod gl_util;
mod gl_rendering;
mod graphics;
mod gravity;
mod high_score_file;
mod input;
mod point2;
mod random;
mod shaders;
mod textures;

use point2::*;
use board::*;
use gl::types::*;
use graphics::{Vertex2, Color, TCVertex2, Rectangle};
use sdl2::controller::Button;

#[derive(Copy, Clone, PartialEq)]
pub enum ProgramState {
    TitleScreen,
    Playing,
    Paused,
    GameOver(f64),
    Landed,
    Holding {time_left: f64, total_time: f64 },
    Fading {time_left: f64, total_time: f64 },
    Matching { time_left: f64 }
}


fn main() {
    let mut last_score = 0;
    let mut high_score = high_score_file::read_high_score();

    let window_size = [352, 520];
    let cell_size = [32., 32.];
    let cell_padding = [0., 0.];
    let key_bindings: std::collections::HashMap<sdl2::keyboard::Keycode, input::Buttons> = hashmap!{
        sdl2::keyboard::Keycode::Left => input::Buttons::Left,
        sdl2::keyboard::Keycode::Right => input::Buttons::Right,
        sdl2::keyboard::Keycode::Down => input::Buttons::Down,
        sdl2::keyboard::Keycode::Z => input::Buttons::CycleUp,
        sdl2::keyboard::Keycode::X => input::Buttons::CycleDown,
        sdl2::keyboard::Keycode::Space => input::Buttons::Start,
        sdl2::keyboard::Keycode::Escape => input::Buttons::Quit
    };

    let controller_bindings: std::collections::HashMap<sdl2::controller::Button, input::Buttons> = hashmap!{
        sdl2::controller::Button::B => input::Buttons::CycleUp,
        sdl2::controller::Button::A => input::Buttons::CycleDown,
        sdl2::controller::Button::DPadLeft => input::Buttons::Left,
        sdl2::controller::Button::DPadRight => input::Buttons::Right,
        sdl2::controller::Button::DPadDown => input::Buttons::Down,
        sdl2::controller::Button::Start => input::Buttons::Start
    };

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);


    let window =
        video_subsystem
            .window("PILLARS!", window_size[0], window_size[1])
            .opengl()
            .position_centered()
            .build()
            .unwrap();
    let gl_context = window.gl_create_context().unwrap();
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let pillar_bytes = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/pillar.png"));
    let pillar_texture = textures::load_png_into_texture(std::io::Cursor::new(&pillar_bytes[..]));
    let charset_bytes = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/charset.png"));
    let charset_texture = textures::load_png_into_texture(std::io::Cursor::new(&charset_bytes[..]));
    let block_bytes = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/block.png"));
    let block_texture = textures::load_png_into_texture(std::io::Cursor::new(&block_bytes[..]));

    let controller_subsystem = sdl_context.game_controller().unwrap();
    let mut controllers = {
        let mut cs = Vec::new();
        if let Ok(n) = controller_subsystem.num_joysticks() {
            for id in 0..n {
                if controller_subsystem.is_game_controller(id) {
                    if let Ok(controller) = controller_subsystem.open(id) {
                        cs.push(controller);
                    }
                }
            }
        }
        cs
    };

    let mut event_pump = sdl_context.event_pump().unwrap();

    event_pump.enable_event(sdl2::event::EventType::ControllerButtonDown);
    event_pump.enable_event(sdl2::event::EventType::ControllerButtonUp);

    let mut game_data = game::GameData::default();
    let mut input_state = input::InputState::default();
    let mut ticks = 0;


    unsafe {
        gl::Viewport(0, 0, 600, 600);
        gl::ClearColor(0.12, 0.12, 0.16, 1.0);
        gl::Enable(gl::BLEND);
        gl::BlendEquationSeparate(gl::FUNC_ADD, gl::FUNC_ADD);
        gl::BlendFuncSeparate(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA, gl::ONE, gl::ZERO);
    }
    
    let mut orthogonal_projection_matrix = graphics::calculate_orthogonal_projection_matrix([600., 600.], [0., 0.]);


    let shaders = [
        gl_util::Shader::from_str(shaders::VERTEX_SHADER_SRC, gl::VERTEX_SHADER).unwrap(),
        gl_util::Shader::from_str(shaders::FRAGMENT_SHADER_SRC, gl::FRAGMENT_SHADER).unwrap()
    ];

    let shader_program = gl_util::link_program(&shaders).unwrap();    
    shader_program.use_program();

    unsafe {
        use std::ffi::CString;                
        let matrix_ref = shader_program.get_uniform(&CString::new("camera_matrix").unwrap());
        gl::UniformMatrix4fv(matrix_ref, 1, gl::FALSE, orthogonal_projection_matrix.as_ptr());
    }

    let mut vertices = Vec::<TCVertex2>::new();
    gl_rendering::push_quad_vertices(&mut vertices, [200., 200.], [128., 128.], graphics::YELLOW);

    let mut vertex_buffer: GLuint = 0;    
    let mut vertex_attributes_array: GLuint = 0;

    unsafe {
        gl::GenBuffers(1, &mut vertex_buffer);
        gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<TCVertex2>()) as GLsizeiptr,
            vertices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW
        );
        
        gl::GenVertexArrays(1, &mut vertex_attributes_array);
        gl::BindVertexArray(vertex_attributes_array);
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            2,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<TCVertex2>() as GLsizei,
            std::ptr::null()
        );
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<TCVertex2>() as GLsizei,
            std::mem::size_of::<Vertex2>() as * const GLvoid
        );
        gl::EnableVertexAttribArray(2);
        gl::VertexAttribPointer(
            2,
            4,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<TCVertex2>() as GLsizei,
            (std::mem::size_of::<Vertex2>() * 2) as * const GLvoid
        );
     
        gl::BindVertexArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    let charset = charset::Charset::new();
    let mut board_vertices = Vec::with_capacity((game_data.board.width() * game_data.board.height() + 20) * 6);

    let target = [64., 64.];

    let border_cell_size = [cell_size[0] + cell_padding[0], cell_size[1] + cell_padding[1]];
    let mut border_vertices = Vec::new();
    let border_color = [0.75, 0.75, 0.75, 1.0];
    for i in 0..game_data.board.width() + 2  {
        gl_rendering::push_quad_vertices(
            &mut border_vertices,
            [target[0] - border_cell_size[0] + border_cell_size[0] * (i as f32), target[1] - border_cell_size[1]],
            border_cell_size,
            border_color);
    }
    for i in 0..game_data.board.height()  {
        gl_rendering::push_quad_vertices(
            &mut border_vertices,
            [target[0] - border_cell_size[0], target[1] + i as f32 * border_cell_size[1]],
            border_cell_size,
            border_color);
        gl_rendering::push_quad_vertices(
            &mut border_vertices,
            [target[0] + ((cell_size[0] + cell_padding[0]) * game_data.board.width() as f32), target[1] + i as f32 * border_cell_size[1]],
            border_cell_size,
            border_color);
    }

    let mut last_ns = time::precise_time_ns() - 5;
    let char_size = [16., 16.];
    let top = (window_size[1] - 1) as f32;
    let left = 0.;
    let right = (window_size[0] - 1) as f32;
    let bottom = 0.;
    let window_rect = Rectangle { position: [0., 0.], size: [(window_size[0] - 1) as f32, (window_size[1] - 1) as f32] };

    let mut program_state = ProgramState::TitleScreen;

    'game_loop: loop {
        
        // things to do every frame
        let current_ns = time::precise_time_ns();
        let mut frame_time_ns = current_ns - last_ns;
        last_ns = current_ns;
        if frame_time_ns == 0 {
            // need a really fast computer for this to matter?
            frame_time_ns = 1;
        };
        let time_delta = (1. / 1_000_000_000.) * (frame_time_ns as f64);
        input_state.store_current();
        for event in event_pump.poll_iter() {
            events::process_sdl_event(&event, &mut input_state, &key_bindings, &controller_bindings, &mut controllers, &controller_subsystem);
        }

        if input_state.down(input::Buttons::Quit) {
            break 'game_loop;
        }

        match program_state {
            ProgramState::TitleScreen => {
                if input_state.just_pressed(input::Buttons::Start) {
                    game_data = game::GameData::default();
                    program_state = ProgramState::Playing;
                }
                let display_strings = {
                    let mut temp = gl_rendering::get_scores_display_strings(game_data.score, high_score, window_rect, char_size);
                    temp.push(("pillars".to_string().into_bytes(), [right * 0.5 - 3.5 * char_size[0], top * 0.5]));
                    temp
                };
                
                let mut charset_vertices = Vec::new();
                for message in display_strings.iter() {
                    charset.push_text_vertices(&mut charset_vertices, &message.0, message.1, char_size, graphics::WHITE);
                }
                unsafe {
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }
                gl_util::draw_textured_colored_quads(
                    &charset_vertices,
                    &shader_program,
                    charset_texture.id(),
                    vertex_buffer,
                    vertex_attributes_array
                );
                window.gl_swap_window();
            },
            ProgramState::Playing => {
                if input_state.just_pressed(input::Buttons::Start) {
                    program_state = ProgramState::Paused;
                    continue 'game_loop;
                }

                if game_data.game_over  {
                    last_score = game_data.score;
                    if high_score < last_score {
                        high_score = last_score;
                    }
                    program_state = ProgramState::GameOver(5.0);
                    continue 'game_loop;
                }

                game::update_game(&mut game_data, &mut program_state, &input_state, time_delta);

                board_vertices.clear();

                let next_column = game_data.next_column;
                gl_rendering::draw_column(
                    &mut board_vertices,
                    next_column,
                    target,
                    cell_size,
                    cell_padding,
                    0.5);
                gl_rendering::draw_board(
                    &mut board_vertices,
                    &game_data.board,
                    game_data.current_column,
                    target,
                    cell_size,
                    cell_padding);

                let display_strings = gl_rendering::get_scores_display_strings(game_data.score, high_score, window_rect, char_size);

                let mut charset_vertices = Vec::new();
                for message in display_strings.iter() {
                    charset.push_text_vertices(&mut charset_vertices, &message.0, message.1, char_size, graphics::WHITE);
                }

                unsafe {
                    gl::Clear(gl::COLOR_BUFFER_BIT);

                    // draw all pillars
                    gl_util::draw_textured_colored_quads(
                        &board_vertices,
                        &shader_program,
                        pillar_texture.id(),
                        vertex_buffer,
                        vertex_attributes_array
                    );

                    gl_util::draw_textured_colored_quads(
                        &border_vertices,
                        &shader_program,
                        block_texture.id(),
                        vertex_buffer,
                        vertex_attributes_array
                    );

                    gl_util::draw_textured_colored_quads(
                        &charset_vertices,
                        &shader_program,
                        charset_texture.id(),
                        vertex_buffer,
                        vertex_attributes_array
                    );
                }
                window.gl_swap_window();

            },
            ProgramState::GameOver(time_left) => {
                let time_left = time_left - time_delta;
                program_state =
                    if time_left < 0.
                    || input_state.just_pressed(input::Buttons::Start) {
                        ProgramState::TitleScreen
                    } else {
                        ProgramState::GameOver(time_left)
                    };
                board_vertices.clear();
                let next_column = game_data.next_column;
                gl_rendering::draw_column(
                    &mut board_vertices,
                    next_column,
                    target,
                    cell_size,
                    cell_padding,
                    0.5);
                gl_rendering::draw_board(
                    &mut board_vertices,
                    &game_data.board,
                    game_data.current_column,
                    target,
                    cell_size,
                    cell_padding);

                let display_strings = {
                    let mut temp = gl_rendering::get_scores_display_strings(game_data.score, high_score, window_rect, char_size);
                    temp.push(("game over".to_string().into_bytes(), [right * 0.5 - 4.5 * char_size[0], top * 0.5]));
                    temp
                };

                let mut charset_vertices = Vec::new();
                for message in display_strings.iter() {

                    charset.push_text_vertices(&mut charset_vertices, &message.0, message.1, char_size, graphics::WHITE);
                    
                }



                unsafe {
                    gl::Clear(gl::COLOR_BUFFER_BIT);

                        // draw all pillars
                        gl_util::draw_textured_colored_quads(
                            &board_vertices,
                            &shader_program,
                            pillar_texture.id(),
                            vertex_buffer,
                            vertex_attributes_array
                        );


                    gl_util::draw_textured_colored_quads(
                        &border_vertices,
                        &shader_program,
                        block_texture.id(),
                        vertex_buffer,
                        vertex_attributes_array
                    );

                    gl_util::draw_textured_colored_quads(
                        &charset_vertices,
                        &shader_program,
                        charset_texture.id(),
                        vertex_buffer,
                        vertex_attributes_array
                    );
                }
                window.gl_swap_window();

            },
            ProgramState::Paused => {
                if input_state.just_pressed(input::Buttons::Start) {
                    program_state = ProgramState::Playing;
                    continue 'game_loop;
                }

                board_vertices.clear();

                let display_strings = 
                  gl_rendering::get_scores_display_strings(game_data.score, high_score, window_rect, char_size);
                

                let mut charset_vertices = Vec::new();
                for message in display_strings.iter() {
                    charset.push_text_vertices(&mut charset_vertices, &message.0, message.1, char_size, graphics::WHITE);
                }


                charset.push_text_vertices(
                    &mut charset_vertices,&"paused".to_string().into_bytes(),
                    [right * 0.5 - 3. * char_size[0], top * 0.5],
                    char_size, graphics::WHITE);

                unsafe {
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }
                gl_util::draw_textured_colored_quads(
                    &border_vertices,
                    &shader_program,
                    block_texture.id(),
                    vertex_buffer,
                    vertex_attributes_array
                );

                gl_util::draw_textured_colored_quads(
                    &charset_vertices,
                    &shader_program,
                    charset_texture.id(),
                    vertex_buffer,
                    vertex_attributes_array
                );
                window.gl_swap_window();
                
            },
            ProgramState::Holding { time_left, total_time } => {
                if time_left < 0.0 {
                    game_data.current_column = game_data.next_column;
                    game_data.next_column = columns::Column::new(game_data.column_spawn_point);
                    game_data.drop_cool_down = -game_data.drop_speed * 0.5;
                    game_data.game_over = board_analysis::check_for_collision(&game_data.board, &game_data.current_column);                    
                    program_state = ProgramState::Playing
                } else {
                    program_state = ProgramState::Holding{ time_left: time_left - time_delta, total_time };
                }
            },
            ProgramState::Landed => {
                if !gravity::drop_jewels(&mut game_data.board) {
                    game_data.matches = board_analysis::scan_for_matches(&game_data.board, game_data.min_gem_line_length);
                    if game_data.matches.is_empty() {
                        program_state = ProgramState::Holding{time_left: 0.25, total_time: 0.25};
                        game_data.score += game_data.score_accumulator;
                        if 0 < game_data.score_accumulator {
                            game_data.last_accumulated_score = game_data.score_accumulator;
                        }
                        game_data.score_accumulator = 0;
                    } else {
                        program_state = ProgramState::Matching{ time_left: 0.1 };
                    }
                }
            },
            ProgramState::Matching {time_left } => {
                program_state = if time_left < 0.0 {
                    ProgramState::Fading { time_left: game_data.matching_time, total_time: game_data.matching_time }                       
                } else {
                    ProgramState::Matching { time_left: time_left - time_delta }
                };
                board_vertices.clear();

                let next_column = game_data.next_column;
                gl_rendering::draw_column(
                    &mut board_vertices,
                    next_column,
                    target,
                    cell_size,
                    cell_padding,
                    0.5);
                gl_rendering::draw_board_highlight_matches(
                    &mut board_vertices,
                    &game_data.board,
                    &game_data.matches,
                    target,
                    cell_size,
                    cell_padding);

                let display_strings = gl_rendering::get_scores_display_strings(game_data.score, high_score, window_rect, char_size);

                let mut charset_vertices = Vec::new();
                for message in display_strings.iter() {
                    charset.push_text_vertices(&mut charset_vertices, &message.0, message.1, char_size, graphics::WHITE);
                }

                unsafe {
                    gl::Clear(gl::COLOR_BUFFER_BIT);

                    // draw all pillars
                    gl_util::draw_textured_colored_quads(
                        &board_vertices,
                        &shader_program,
                        pillar_texture.id(),
                        vertex_buffer,
                        vertex_attributes_array
                    );

                    gl_util::draw_textured_colored_quads(
                        &border_vertices,
                        &shader_program,
                        block_texture.id(),
                        vertex_buffer,
                        vertex_attributes_array
                    );

                    gl_util::draw_textured_colored_quads(
                        &charset_vertices,
                        &shader_program,
                        charset_texture.id(),
                        vertex_buffer,
                        vertex_attributes_array
                    );
                }
                window.gl_swap_window();
            },
            ProgramState::Fading { time_left, total_time } => {
                if time_left < 0.0 {
                    for p in &game_data.matches {
                        game_data.score_accumulator += game_data.level + 1;
                        game_data.board[*p] = None;
                        program_state = ProgramState::Landed;                        
                    }
                } else {
                    program_state = ProgramState::Fading { time_left: time_left - time_delta, total_time };
                }

                board_vertices.clear();

                let next_column = game_data.next_column;
                gl_rendering::draw_column(
                    &mut board_vertices,
                    next_column,
                    target,
                    cell_size,
                    cell_padding,
                    0.5);
                let alpha = time_left / total_time;
                gl_rendering::draw_board_fade_matches(
                    &mut board_vertices,
                    &game_data.board,
                    &game_data.matches,
                    alpha as f32,
                    target,
                    cell_size,
                    cell_padding);

                let display_strings = gl_rendering::get_scores_display_strings(game_data.score, high_score, window_rect, char_size);

                let mut charset_vertices = Vec::new();
                for message in display_strings.iter() {
                    charset.push_text_vertices(&mut charset_vertices, &message.0, message.1, char_size, graphics::WHITE);
                }

                unsafe {
                    gl::Clear(gl::COLOR_BUFFER_BIT);

                    // draw all pillars
                    gl_util::draw_textured_colored_quads(
                        &board_vertices,
                        &shader_program,
                        pillar_texture.id(),
                        vertex_buffer,
                        vertex_attributes_array
                    );

                    gl_util::draw_textured_colored_quads(
                        &border_vertices,
                        &shader_program,
                        block_texture.id(),
                        vertex_buffer,
                        vertex_attributes_array
                    );

                    gl_util::draw_textured_colored_quads(
                        &charset_vertices,
                        &shader_program,
                        charset_texture.id(),
                        vertex_buffer,
                        vertex_attributes_array
                    );
                }
                window.gl_swap_window();


            }
            _ => {}

        }
    }
    high_score_file::write_high_score(high_score);
}



