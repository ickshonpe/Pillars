use board::Board;
use columns::Column;
use point2::P2;
use std::collections::HashSet;

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
    pub last_accumulated_score: u64,
    pub grounded_time: f64,
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
            game_over: false,
            grounded_time: 0.5,
        }
    }
}
