use game_data::GameData;
use game_state::GameState;
use game_update;
use input::Buttons;
use input::InputState;
use point2::P2;
use timer::Timer;
use gl_rendering;

#[derive(Copy, Clone, Debug)]
pub struct HighScore {
    last_score: u64,
    high_score: u64,
}

impl HighScore {
    pub fn new(high_score: u64) -> HighScore {
        HighScore {
            last_score: 0,
            high_score,
        }
    }
    pub fn update(&self, new_score: u64) -> HighScore {
        let high_score = if self.high_score < new_score {
            new_score
        } else {
            self.high_score
        };
        HighScore {
            high_score,
            last_score: new_score,
        }
    }

    pub fn last_score(&self) -> u64 {
        self.last_score
    }

    pub fn value(&self) -> u64 {
        self.high_score
    }
}

pub struct TitleScreen {
    high_scores: HighScore,
}
pub struct Playing {
    high_scores: HighScore,
    game_data: GameData,
}
pub struct Paused {
    high_scores: HighScore,
    previous: Box<GameState>,
}

pub struct GameOver {
    high_scores: HighScore,
    time_left: f64,
    fade_time: f64,
    fading: Vec<(P2, f32)>,
    pillars: Vec<(P2, f32)>,
}

pub struct Grounded {
    high_scores: HighScore,
    game_data: GameData,
}

pub struct Landed {
    high_scores: HighScore,
    game_data: GameData,
}
pub struct Holding {
    high_scores: HighScore,
    game_data: GameData,
    time_left: f64,
    total_time: f64,
}
pub struct Fading {
    high_scores: HighScore,
    game_data: GameData,
    time_left: f64,
    total_time: f64,
}
pub struct Matching {
    high_scores: HighScore,
    game_data: GameData,
    time_left: f64,
}

use graphics;

impl TitleScreen {
    pub fn new(high_score: u64) -> Self {
        TitleScreen {
            high_scores: HighScore::new(high_score)
        }
    }
}

impl GameState for TitleScreen {
    fn update(mut self: Box<Self>, time_delta: f64, input_state: &InputState) -> Box<GameState> {
        if input_state.just_pressed(Buttons::Start) {
            let game_data = GameData::default();
            let next_state = Playing {
                high_scores: self.high_scores,
                game_data,
            };
            Box::new(next_state)
        } else {
            self
        }
    }

    fn draw(&self, ctx: &graphics::GraphicsContext) {
        use gl;
        use gl_rendering;
        use gl_util;
        use graphics;
        let display_strings = {
            let mut temp = gl_rendering::get_scores_display_strings(
                self.high_scores.last_score,
                self.high_scores.high_score,
                ctx.window_rect,
                ctx.char_size,
            );
            let right = ctx.window_rect.right();
            let top = ctx.window_rect.top();
            temp.push((
                "pillars".to_string().into_bytes(),
                [right * 0.5 - 3.5 * ctx.char_size[0], top * 0.5],
            ));
            temp
        };

        let mut charset_vertices = Vec::new();
        for message in &display_strings {
            ctx.charset.push_text_vertices(
                &mut charset_vertices,
                &message.0,
                message.1,
                ctx.char_size,
                graphics::WHITE,
            );
        }
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        gl_util::draw_textured_colored_quads(
            &charset_vertices,
            &ctx.shader_program,
            ctx.charset_texture.id(),
            ctx.vertex_buffer,
            ctx.vertex_attributes_array,
        );
        ctx.window.gl_swap_window();
    }
}

impl GameState for Playing {
    fn update(mut self: Box<Self>, time_delta: f64, input_state: &InputState) -> Box<GameState> {
        if input_state.just_pressed(Buttons::Start) {
            return Box::new(Paused {
                high_scores: self.high_scores,
                previous: self,
            });
        }

        if self.game_data.game_over {
            self.high_scores = self.high_scores.update(self.game_data.score);

            let gameover_pillars = {
                let mut temp = Vec::new();
                let board = &self.game_data.board;
                use point2::Size2;
                for x in 0..board.width() {
                    for y in 0..board.height() {
                        let p = P2::new(x, y);
                        if board[p].is_some() {
                            temp.push((p, 1.0_f32));
                        }
                    }
                }
                use rand::{thread_rng, Rng};
                thread_rng().shuffle(&mut temp);
                temp
            };
            //program_state = ProgramState::GameOver(10.0, 0.2, Vec::new(), gameover_pillars);

            let next_state = GameOver {
                high_scores: self.high_scores,
                time_left: 10.,
                fade_time: 0.2,
                fading: Vec::new(),
                pillars: gameover_pillars,
            };
            return Box::new(next_state);
        }
        if ::game_update::update_game(&mut self.game_data, &input_state, time_delta) {
            let next_state = Grounded {
                high_scores: self.high_scores,
                game_data: self.game_data,
            };
            Box::new(next_state)
        } else {
            self
        }
    }

    fn draw(&self, ctx: &graphics::GraphicsContext) {
        let mut board_vertices = Vec::new();
        let next_column = self.game_data.next_column;
        gl_rendering::draw_column(
            &mut board_vertices,
            next_column,
            ctx.target,
            ctx.cell_size,
            ctx.cell_padding,
            0.5,
        );
        gl_rendering::draw_board(
            &mut board_vertices,
            &self.game_data.board,
            Some(self.game_data.current_column),
            ctx.target,
            ctx.cell_size,
            ctx.cell_padding,
        );

         let display_strings = gl_rendering::get_scores_display_strings(
                    self.game_data.score,
                    self.high_scores.value(),
                    ctx.window_rect,
                    ctx.char_size,
                );

                let mut charset_vertices = Vec::new();
                for message in &display_strings {
                    ctx.charset.push_text_vertices(
                        &mut charset_vertices,
                        &message.0,
                        message.1,
                        ctx.char_size,
                        graphics::WHITE,
                    );
                }


         unsafe {
             use gl;
             use gl_util;
                    gl::Clear(gl::COLOR_BUFFER_BIT);

                    // draw all pillars
                    gl_util::draw_textured_colored_quads(
                        &board_vertices,
                        &ctx.shader_program,
                        ctx.pillar_texture.id(),
                        ctx.vertex_buffer,
                        ctx.vertex_attributes_array,
                    );

                    gl_util::draw_textured_colored_quads(
                        &ctx.border_vertices,
                        &ctx.shader_program,
                        ctx.block_texture.id(),
                        ctx.vertex_buffer,
                        ctx.vertex_attributes_array,
                    );

                    gl_util::draw_textured_colored_quads(
                        &charset_vertices,
                        &ctx.shader_program,
                        ctx.charset_texture.id(),
                        ctx.vertex_buffer,
                        ctx.vertex_attributes_array,
                    );
                }


        ctx.window.gl_swap_window();
    }
}

impl GameState for Paused {
    fn update(self: Box<Self>, _time_delta: f64, input_state: &InputState) -> Box<GameState> {
        if input_state.just_pressed(Buttons::Start) {
            self.previous
        } else {
            self
        }
    }

    fn draw(&self, ctx: &graphics::GraphicsContext) {

    }
}

impl GameState for GameOver {
    fn update(mut self: Box<Self>, time_delta: f64, input_state: &InputState) -> Box<GameState> {
        self.time_left -= time_delta;
        self.fade_time -= time_delta;
        if self.fade_time < 0. {
            if let Some(next) = self.pillars.pop() {
                self.fading.push(next);
            }
            self.fade_time = 0.2;
        }
        for fader in &mut self.fading {
            fader.1 -= time_delta as f32;
        }

        if self.time_left < 0. || input_state.just_pressed(Buttons::Start) {
            // Box::new(TitleScreen {
            //     high_scores: self.high_scores,

            // })
            self
        } else {
            self
        }
    }

    fn draw(&self, ctx: &graphics::GraphicsContext) {}
}

impl GameState for Holding {
    fn update(self: Box<Self>, time_delta: f64, input_state: &InputState) -> Box<GameState> {
        self
    }

    fn draw(&self, ctx: &graphics::GraphicsContext) {}
}

impl GameState for Landed {
    fn update(self: Box<Self>, time_delta: f64, input_state: &InputState) -> Box<GameState> {
        self
    }

    fn draw(&self, ctx: &graphics::GraphicsContext) {}
}

impl GameState for Grounded {
    fn update(mut self: Box<Self>, time_delta: f64, input_state: &InputState) -> Box<GameState> {
        if input_state.just_pressed(Buttons::Start) {
            // program_state = ProgramState::Paused;
            // continue 'game_loop;
            return Box::new(Paused {
                high_scores: self.high_scores,
                previous: self,
            });
        }

        if self.game_data.game_over {
            self.high_scores = self.high_scores.update(self.game_data.score);

            let gameover_pillars = {
                let mut temp = Vec::new();
                let board = &self.game_data.board;
                use point2::Size2;
                for x in 0..board.width() {
                    for y in 0..board.height() {
                        let p = P2::new(x, y);
                        if board[p].is_some() {
                            temp.push((p, 1.0_f32));
                        }
                    }
                }
                use rand::{thread_rng, Rng};
                thread_rng().shuffle(&mut temp);
                temp
            };
            let next_state = GameOver {
                high_scores: self.high_scores,
                time_left: 10.,
                fade_time: 0.2,
                fading: Vec::new(),
                pillars: gameover_pillars,
            };
            return Box::new(next_state);
        }

        //game_update::update_game_grounded(&mut game_data, &input_state, time_delta);
        //let game_board = &mut game_data.board;
        self.game_data.rotate_cool_down -= time_delta;
        if self.game_data.rotate_cool_down < 0.0 {
            if input_state.just_pressed(Buttons::CycleUp) {
                self.game_data.current_column.cycle_up();
                self.game_data.rotate_cool_down = self.game_data.rotate_speed;
            } else if input_state.just_pressed(Buttons::CycleDown) {
                self.game_data.current_column.cycle_down();
                self.game_data.rotate_cool_down = self.game_data.rotate_speed;
            }
        }

        self.game_data.grounded_time -= time_delta;
        if self.game_data.grounded_time < 0. {
            for i in 0..3 {
                let p = self.game_data.current_column.position;
                let jewel = self.game_data.current_column[i];
                self.game_data.board[p.x][p.y + i] = Some(jewel);
            }
            let next_state = Landed {
                high_scores: self.high_scores,
                game_data: self.game_data,
            };
            return Box::new(next_state);
        }

        self
    }

    fn draw(&self, ctx: &graphics::GraphicsContext) {}
}

impl GameState for Fading {
    fn update(mut self: Box<Self>, time_delta: f64, input_state: &InputState) -> Box<GameState> {
        if self.time_left < 0.0 {
            let matches = self.game_data.matches.clone();
            for p in matches {
                self.game_data.score_accumulator += self.game_data.level + 1;
                self.game_data.board[p] = None;
            }

            let next_state = Landed {
                high_scores: self.high_scores,
                game_data: self.game_data,
            };
            Box::new(next_state)
        } else {
            self.time_left -= time_delta;
            self
        }
    }

    fn draw(&self, ctx: &graphics::GraphicsContext) {}
}

impl GameState for Matching {
    fn update(mut self: Box<Self>, time_delta: f64, input_state: &InputState) -> Box<GameState> {
        if self.time_left < 0.0 {
            let matching_time = self.game_data.matching_time;
            Box::new(Fading {
                high_scores: self.high_scores,
                game_data: self.game_data,
                time_left: matching_time,
                total_time: matching_time,
            })
        } else {
            self.time_left -= time_delta;
            self
        }
    }

    fn draw(&self, ctx: &graphics::GraphicsContext) {}
}
