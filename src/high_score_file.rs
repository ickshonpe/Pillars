use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

const HIGH_SCORE_FILE: &'static str = ".high_scores";

pub fn read_high_score() -> u64 {
    let path = Path::new(HIGH_SCORE_FILE);
    let mut file = match File::open(path) {
        Err(_) => return 0,
        Ok(file) => file
    };
    let mut score_string = String::new();
    match file.read_to_string(&mut score_string) {
        Err(_) => return 0,
        Ok(_) => {}
    };
    score_string.parse().unwrap_or(0)
}

pub fn write_high_score(high_score: u64) {
    let path = Path::new(HIGH_SCORE_FILE);
    let mut file = match File::create(path) {
        Err(_) => return,
        Ok(file) => file
    };
    match file.write_all(&format!("{}", high_score).into_bytes()) {
        Err(_) => {},
        Ok(_) => {}
    }
}