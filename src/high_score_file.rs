use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

const HIGH_SCORE_FILE: &'static str = ".high_scores";

/// attempt to load the high score from a file
/// failure returns a high score of 0.
pub fn read_high_score() -> u64 {
    let path = Path::new(HIGH_SCORE_FILE);
    if let Ok(mut file) = File::open(path) {
        let mut score_string = String::new();
        file.read_to_string(&mut score_string)
            .map(|_| score_string.parse().unwrap_or(0))
            .unwrap_or(0)
    } else {
        0
    }
}

/// attempt to save current high score to a file
/// fails silently - doesn't matter that much
pub fn write_high_score(high_score: u64) {
    let path = Path::new(HIGH_SCORE_FILE);
    if let Ok(mut file) = File::create(HIGH_SCORE_FILE) {
        file.write_all(&format!("{}", high_score).into_bytes());
    }
}