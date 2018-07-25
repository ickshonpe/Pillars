use std::collections::HashSet;
use point2::*;
use board::*;
use board_analysis::*;
use columns::*;
use gravity;
use input::InputState;


pub struct GameData {
    pub board: Board,
    pub min_gem_line_length: usize,
    pub column_spawn_point: P2,
    pub next_column: Column,
    pub current_column: Column,
    pub level: u64,
    pub score: u64,
    pub level_time: f64,
    pub drop_speed: f64,
    pub speed_up: f64,
    pub score_accumulator: u64,
    pub score_combo: u64,
    pub move_speed: f64,
    pub rotate_speed: f64,
    pub move_cool_down: f64,
    pub rotate_cool_down: f64,
    pub drop_cool_down: f64,
    pub matching_time: f64,
    pub matches: HashSet<P2>,
    pub game_over: bool,
    pub last_accumulated_score: u64

}

impl Default for GameData {
    fn default() -> Self {
        GameData {
            board: Board::new(7, 13),
            min_gem_line_length: 3,
            column_spawn_point: P2::new(3, 10),
            next_column: Column::new(P2::new(3, 10)),
            current_column: Column::new(P2::new(3, 10)),
            level: 0,
            score: 0,
            level_time: 20.0,
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
            matches: HashSet::new(),
            last_accumulated_score: 0,
            game_over: false
        }
    }
}

use input::Buttons;
pub fn update_game(game_data: &mut GameData, program_state: &mut ::ProgramState, input: &InputState, time_delta: f64) {
    let game_board = &mut game_data.board;
    game_data.drop_cool_down += time_delta;
    game_data.move_cool_down -= time_delta;
    game_data.rotate_cool_down -= time_delta;
    if game_data.level_time < 0.0 {
        game_data.level += 1;
        game_data.level_time = 20.0;
        game_data.drop_speed *= 0.9;
    }
    game_data.level_time -= time_delta;
    if game_data.move_cool_down < 0.0 {
        if input.down(Buttons::Left) {
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
                *program_state = ::ProgramState::Landed;
    } else if game_data.current_column.position.y == 0 {
        *program_state = ::ProgramState::Landed;
    }

    if *program_state == ::ProgramState::Landed {
        let p = game_data.current_column.position;
        for i in 0..3 {
            let jewel = game_data.current_column[i];
            game_board[p.x][p.y + i] = Some(jewel);
        }
    }
}

pub fn update_game_grounded(game_data: &mut GameData, program_state: &mut ::ProgramState, input: &InputState, time_delta: f64) {
    let game_board = &mut game_data.board;
    game_data.drop_cool_down += time_delta;
    game_data.move_cool_down -= time_delta;
    game_data.rotate_cool_down -= time_delta;
    if game_data.level_time < 0.0 {
        game_data.level += 1;
        game_data.level_time = 20.0;
        game_data.drop_speed *= 0.9;
    }
    game_data.level_time -= time_delta;
    if game_data.move_cool_down < 0.0 {
        if input.down(Buttons::Left) {
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
                *program_state = ::ProgramState::Landed;
    } else if game_data.current_column.position.y == 0 {
        *program_state = ::ProgramState::Landed;
    }

    if *program_state == ::ProgramState::Landed {
        let p = game_data.current_column.position;
        for i in 0..3 {
            let jewel = game_data.current_column[i];
            game_board[p.x][p.y + i] = Some(jewel);
        }
    }
}