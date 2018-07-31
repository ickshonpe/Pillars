use high_score_file;
use input;
use gl;
use textures;
use game_data;
use game_state;
use states;
use graphics;
use gl_util;
use shaders;
use gl_rendering;
use board::*;
use gl::types::*;
use graphics::{Color, Rectangle, TCVertex2, Vertex2};
use point2::*;
use sdl2::controller::Button;
use game;
use ::ProgramState;
use sdl2;
use std;
use charset;
use time;
use events;
use game_state::GameState;

pub fn main() {
    let mut last_score = 0;
    let mut high_score = high_score_file::read_high_score();

    let window_size = [352, 520];
    let cell_size = [32., 32.];
    let cell_padding = [0., 0.];
    let key_bindings: std::collections::HashMap<sdl2::keyboard::Keycode, input::Buttons> = hashmap! {
        sdl2::keyboard::Keycode::Left => input::Buttons::Left,
        sdl2::keyboard::Keycode::Right => input::Buttons::Right,
        sdl2::keyboard::Keycode::Down => input::Buttons::Down,
        sdl2::keyboard::Keycode::Z => input::Buttons::CycleUp,
        sdl2::keyboard::Keycode::X => input::Buttons::CycleDown,
        sdl2::keyboard::Keycode::Space => input::Buttons::Start,
        sdl2::keyboard::Keycode::Escape => input::Buttons::Quit
    };

    let controller_bindings: std::collections::HashMap<
        sdl2::controller::Button,
        input::Buttons,
    > = hashmap! {
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

    let window = video_subsystem
        .window("PILLARS!", window_size[0], window_size[1])
        .opengl()
        .position_centered()
        .build()
        .unwrap();

    let gl_context = window.gl_create_context().unwrap();
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let pillar_bytes = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/pillar.png"));
    let pillar_texture = textures::Texture::from_png(std::io::Cursor::new(&pillar_bytes[..]));
    let charset_bytes = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/charset.png"));
    let charset_texture = textures::Texture::from_png(std::io::Cursor::new(&charset_bytes[..]));
    let block_bytes = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/block.png"));
    let block_texture = textures::Texture::from_png(std::io::Cursor::new(&block_bytes[..]));

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

    let mut game_data = game_data::GameData::default();
    let mut input_state = input::InputState::default();
    let mut ticks = 0;

    unsafe {
        gl::Viewport(0, 0, 600, 600);
        gl::ClearColor(0.12, 0.12, 0.16, 1.0);
        gl::Enable(gl::BLEND);
        gl::BlendEquationSeparate(gl::FUNC_ADD, gl::FUNC_ADD);
        gl::BlendFuncSeparate(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA, gl::ONE, gl::ZERO);
    }

    let mut orthogonal_projection_matrix =
        graphics::calculate_orthogonal_projection_matrix([600., 600.], [0., 0.]);

    let shaders = [
        gl_util::Shader::from_str(shaders::VERTEX_SHADER_SRC, gl::VERTEX_SHADER).unwrap(),
        gl_util::Shader::from_str(shaders::FRAGMENT_SHADER_SRC, gl::FRAGMENT_SHADER).unwrap(),
    ];

    let shader_program = gl_util::link_program(&shaders).unwrap();
    shader_program.use_program();

    unsafe {
        use std::ffi::CString;
        let matrix_ref = shader_program.get_uniform(&CString::new("camera_matrix").unwrap());
        gl::UniformMatrix4fv(
            matrix_ref,
            1,
            gl::FALSE,
            orthogonal_projection_matrix.as_ptr(),
        );
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
            gl::STATIC_DRAW,
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
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<TCVertex2>() as GLsizei,
            std::mem::size_of::<Vertex2>() as *const GLvoid,
        );
        gl::EnableVertexAttribArray(2);
        gl::VertexAttribPointer(
            2,
            4,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<TCVertex2>() as GLsizei,
            (std::mem::size_of::<Vertex2>() * 2) as *const GLvoid,
        );

        gl::BindVertexArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    let charset = charset::Charset::new();
//    let mut board_vertices =
//        Vec::with_capacity((game_data.board.width() * game_data.board.height() + 20) * 6);

    let target = [64., 64.];

    let border_cell_size = [
        cell_size[0] + cell_padding[0],
        cell_size[1] + cell_padding[1],
    ];
    let mut border_vertices = Vec::new();
    let border_color = [0.75, 0.75, 0.75, 1.0];
    for i in 0..game_data.board.width() + 2 {
        gl_rendering::push_quad_vertices(
            &mut border_vertices,
            [
                target[0] - border_cell_size[0] + border_cell_size[0] * (i as f32),
                target[1] - border_cell_size[1],
            ],
            border_cell_size,
            border_color,
        );
    }
    for i in 0..game_data.board.height() {
        gl_rendering::push_quad_vertices(
            &mut border_vertices,
            [
                target[0] - border_cell_size[0],
                target[1] + i as f32 * border_cell_size[1],
            ],
            border_cell_size,
            border_color,
        );
        gl_rendering::push_quad_vertices(
            &mut border_vertices,
            [
                target[0] + ((cell_size[0] + cell_padding[0]) * game_data.board.width() as f32),
                target[1] + i as f32 * border_cell_size[1],
            ],
            border_cell_size,
            border_color,
        );
    }

    let mut last_ns = time::precise_time_ns() - 5;
    let char_size = [16., 16.];
    let top = (window_size[1] - 1) as f32;
    let left = 0.;
    let right = (window_size[0] - 1) as f32;
    let bottom = 0.;
    let window_rect = Rectangle {
        position: [0., 0.],
        size: [(window_size[0] - 1) as f32, (window_size[1] - 1) as f32],
    };

    let mut game_state = Box::new(states::TitleScreen::new(high_score)) as Box<GameState> ;

    let ctx = graphics::GraphicsContext {
        window_rect,
        char_size,
        charset,
        shader_program,
        charset_texture,
        vertex_buffer,
        vertex_attributes_array,
        window
    };

    'game_loop: loop {
        // things to do every frame
        std::thread::sleep(std::time::Duration::from_millis(2)); // I guess we better not use *all* the cpu
        let current_ns = time::precise_time_ns();
        let mut frame_time_ns = current_ns - last_ns;
        last_ns = current_ns;
        if frame_time_ns == 0 {
            // don't need to bother with this
            frame_time_ns = 1;
        };
        let time_delta = (1. / 1E9/* 1_000_000_000. */) * (frame_time_ns as f64);
        input_state.store_current();
        for event in event_pump.poll_iter() {
            events::process_sdl_event(
                &event,
                &mut input_state,
                &key_bindings,
                &controller_bindings,
                &mut controllers,
                &controller_subsystem,
            );
        }

        game_state = game_state.update(time_delta, &input_state);
        game_state.draw(&ctx);

        if input_state.down(input::Buttons::Quit) {
            break 'game_loop;
        }


    }
    high_score_file::write_high_score(high_score);
}
