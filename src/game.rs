use std::collections::HashSet;
use point2::*;
use board::*;
use board_analysis::*;
use columns::*;
use gravity;
use input::InputState;

#[derive(PartialEq)]
pub enum GameState {
    Playing,
    Dropped,
    Matching(f64)
}

pub struct GameData {
    pub game_board: Board,
    min_gem_line_length: usize,
    column_spawn_point: P2,
    pub next_column: Column,
    pub current_column: Column,
    pub level: u64,
    pub score: u64,
    level_time: f64,
    pub high_score: u64,
    drop_speed: f64,
    speed_up: f64,
    score_accumulator: u64,
    score_combo: u64,
    move_speed: f64,
    rotate_speed: f64,
    move_cool_down: f64,
    rotate_cool_down: f64,
    drop_cool_down: f64,
    matching_time: f64,
    pub game_state: GameState,
    pub matches: HashSet<P2>,
    pub game_over: bool,
    pub last_accumalated_score: u64

}

impl Default for GameData {
    fn default() -> Self {
        GameData {
            game_board: Board::new(7, 13),
            min_gem_line_length: 3,
            column_spawn_point: P2::new(3, 10),
            next_column: Column::new(P2::new(3, 10)),
            current_column: Column::new(P2::new(3, 10)),
            level: 0,
            score: 0,
            level_time: 20.0,
            high_score: 0,
            speed_up: 0.05,
            score_accumulator: 0,
            drop_speed: 0.5,
            score_combo: 0,
            move_speed: 0.1,
            rotate_speed: 0.1,
            drop_cool_down: 1.0,
            move_cool_down: 0.0,
            rotate_cool_down: 0.0,
            matching_time: 0.3,
            game_state: GameState::Playing,
            matches: HashSet::new(),
            last_accumalated_score: 0,
            game_over: false
        }
    }
}

use input::Buttons;
pub fn update_game(game_data: &mut GameData, input: &InputState, time_delta: f64) {
    let game_board = &mut game_data.game_board;
    game_data.drop_cool_down  += time_delta;
    game_data.move_cool_down -= time_delta;
    game_data.rotate_cool_down -= time_delta;
    match game_data.game_state {
        GameState::Playing => {
            if game_data.level_time < 0.0 {
                game_data.level += 1;
                game_data.level_time = 20.0;
                game_data.drop_speed *= 0.9;
            }
            game_data.level_time -= time_delta;
            if game_data.move_cool_down < 0.0 {
                if input.down(Buttons::Left)  {
                    if game_data.current_column.position.x > 0 {
                        game_data.current_column.position.x -= 1;
                        if check_for_collision(&game_board, &game_data.current_column) {
                            game_data.current_column.position.x += 1;
                        } else {
                            game_data.move_cool_down = game_data.move_speed;
                        }
                    }
                } else if input.down(Buttons::Right)
                    && game_data.current_column.position.x < game_board.width() - 1 {
                        game_data.current_column.position.x += 1;
                        if check_for_collision(&game_board, &game_data.current_column) {
                            game_data.current_column.position.x -= 1;
                        } else {
                            game_data.move_cool_down = game_data.move_speed;
                        }
                }
            }
            if game_data.rotate_cool_down < 0.0 {
                if input.just_pressed(Buttons::CycleUp) {
                    game_data.current_column.cycle_up();
                    game_data.rotate_cool_down = game_data.rotate_speed;
                } else if input.just_pressed(Buttons::CycleDown) {
                    game_data.current_column.cycle_down();
                    game_data.rotate_cool_down = game_data.rotate_speed;
                }
            }
            if game_data.drop_cool_down > game_data.drop_speed
                || (input.down(Buttons::Down) && game_data.drop_cool_down > 0.05) {
                game_data.current_column.position.y -= 1;
                game_data.drop_cool_down = 0.0;
            }

            if check_for_collision(&game_board, &game_data.current_column) {
                game_data.current_column.position.y += 1;
                game_data.game_state = GameState::Dropped;
            } else if game_data.current_column.position.y == 0 {
                game_data.game_state = GameState::Dropped;
            }

            if game_data.game_state == GameState::Dropped {
                let p = game_data.current_column.position;
                for i in 0..3 {
                    let jewel = game_data.current_column[i];
                    game_board[p.x][p.y + i] = Some(jewel);
                }
            }
        },
        GameState::Dropped => {
            if !gravity::drop_jewels(game_board) {
                game_data.matches = scan_for_matches(&game_board, game_data.min_gem_line_length);
                if game_data.matches.is_empty() {
                        game_data.game_state = GameState::Playing;
                        game_data.score += game_data.score_accumulator;
                        game_data.last_accumalated_score = game_data.score_accumulator;
                        game_data.score_accumulator = 0;
                        game_data.current_column = game_data.next_column;
                        game_data.next_column = Column::new(game_data.column_spawn_point);
                        game_data.drop_cool_down = -game_data.drop_speed * 0.5;
                        game_data.game_over = check_for_collision(&game_board, &game_data.current_column);
                } else {
                    game_data.game_state = GameState::Matching(game_data.matching_time);
                }
            }
        }
        GameState::Matching(time_left) => {
            if time_left < 0.0 {
                for p in &game_data.matches {
                    game_data.score_accumulator += game_data.score_accumulator + (game_data.level + 1);
                    game_board[*p] = None;
                    game_data.game_state = GameState::Dropped;
                }
            } else {
                game_data.game_state = GameState::Matching(time_left - time_delta);
            }

        }
    }
}
