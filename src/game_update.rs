use board_analysis::*;
use game_data::GameData;
use input::InputState;
use point2::*;

use input::Buttons;
pub fn update_game(game_data: &mut GameData, input: &InputState, time_delta: f64) -> bool {
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
            && game_data.current_column.position.x < game_board.width() - 1
        {
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
        || (input.down(Buttons::Down) && game_data.drop_cool_down > 0.05)
    {
        game_data.current_column.position.y -= 1;
        game_data.drop_cool_down = 0.0;
    }

    if check_for_collision(&game_board, &game_data.current_column) {
        game_data.current_column.position.y += 1;
        //                *program_state = ::ProgramState::Grounded;
        game_data.grounded_time = 0.2;
        true
    } else if game_data.current_column.position.y == 0 {
        //  *program_state = ::ProgramState::Grounded;
        game_data.grounded_time = 0.2;
        true
    } else {
        false
    }
}

// pub fn update_game_grounded(game_data: &mut GameData, input: &InputState, time_delta: f64) -> bool {
//     let game_board = &mut game_data.board;
//     game_data.rotate_cool_down -= time_delta;
//     if game_data.rotate_cool_down < 0.0 {
//         if input.just_pressed(Buttons::CycleUp) {
//             game_data.current_column.cycle_up();
//             game_data.rotate_cool_down = game_data.rotate_speed;
//         } else if input.just_pressed(Buttons::CycleDown) {
//             game_data.current_column.cycle_down();
//             game_data.rotate_cool_down = game_data.rotate_speed;
//         }
//     }

//     game_data.grounded_time -= time_delta;
//     if game_data.grounded_time < 0. {
//         for i in 0..3 {
//             let p = game_data.current_column.position;
//             let jewel = game_data.current_column[i];
//             game_board[p.x][p.y + i] = Some(jewel);
//         }

//     }
// }
