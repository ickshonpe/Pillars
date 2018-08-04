use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const HIGH_SCORE_FILE: &str = ".high_scores";

/// attempt to load the high score from a file
/// fails silently and returns a high score of 0.
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

/// attempt to write current high score to a file
/// fails silently without saving
pub fn write_high_score(high_score: u64) {
    let path = Path::new(HIGH_SCORE_FILE);
    if let Ok(mut file) = File::create(path) {
        let _ = file.write_all(&format!("{}", high_score).into_bytes());
    }    
}
