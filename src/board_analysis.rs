use std::collections::HashSet;
use board_partitioning::*;
use board::*;
use point2::*;
use columns::*;

pub fn scan_for_matches(board: &Board, min_gem_line_length: usize) -> HashSet<P2> {
    if min_gem_line_length < 2 {
        panic!("line length {} is too short", min_gem_line_length);
    }
    let lines: Vec<Vec<P2>> = 
        get_all_lines(board).into_iter().filter(|line| line.len() >= min_gem_line_length).collect();
    
    let mut matches: HashSet<P2> = HashSet::new();    
    for line in lines {          
        let jewels: Vec<Option<Jewel>> = line.iter().map(|p: &P2| board[*p]).collect();
        let xs = scan_line_for_matches(&jewels, min_gem_line_length);
        for i in xs {
            matches.insert(line[i]);
        }
        
    }
    matches
}

pub fn scan_line_for_matches(jewels: &[Option<Jewel>], match_length: usize) -> HashSet<usize> {
    let mut matches = HashSet::new();
    let mut x = 0;
    while x < jewels.len() {
        if let Some(current_jewel) = jewels[x] {
            let mut matching = vec![x];
            x += 1;
            while x < jewels.len() {
                if let Some(next_jewel) = jewels[x] {
                    if current_jewel == next_jewel {
                        matching.push(x);
                    } else {
                        if matching.len() >= match_length {
                            for i in &matching {
                                matches.insert(*i);
                            }
                        }
                        break;
                    }
                } else {
                    if matching.len() >= match_length {
                        for i in &matching {
                            matches.insert(*i);
                        }
                    }
                    break;
                }
                if x == jewels.len() - 1
                    && matching.len() >= match_length {
                        for i in &matching {
                            matches.insert(*i);
                        }
                }
                x += 1;
            }
        } else {
            x += 1;
        }
    }
    matches
}

pub fn check_for_collision(board: &Board, column: &Column) -> bool {
    let p = column.position;
    if board[p] != None { return true }
    if board[p.trans_up()] != None { return true }
    if board[p.trans_up().trans_up()] != None { return true }
    false
}


#[cfg(test)]
#[test]
fn test_row_scan() {
    use Jewel::*;
    let test_row1 = [Some(graphics::Red); 6];
    let test_row2 = [None; 6];
    let result1 = scan_line_for_matches(&test_row1, 3);
    let result2 = scan_line_for_matches(&test_row2, 3);
    assert!(result1.len() == 6);
    assert!(result2.len() == 0);
    for i in 0..6 {
        assert!(result1.contains(&i));
        assert!(!result2.contains(&i));
    }
}

#[cfg(test)]
fn fill_red(board: &mut Board) {    
    for x in 0..board.width() {
        for y in 0..board.height() {
            board[x][y] = Some(Jewel::Red);        
        }
    }        
}



#[cfg(test)]
#[test]
fn test_analysis_1() {
    let mut board = Board::new(7, 13);
    fill_red(&mut board);
    let matches = scan_for_matches(&board, 3);
    for x in 0..board.width() {
        for y in 0..board.height() {
            assert!(matches.contains(&P2 { x, y }));
        }
    }
}

#[cfg(test)]
#[test]
fn test_analysis_2() {
    let mut board: Board = Board::new(7, 13);
    let mut c = 1;
    for jewel in Jewel::all_jewels().iter() {
        for y in 10..13 {
            board[c][y] = Some(*jewel);
        }
        c += 1;
    }
    let matches = scan_for_matches(&board, 3);
    for i in 1..6 {
        assert!(matches.contains(&P2::new(i, board.height() - 1)));
    }
}