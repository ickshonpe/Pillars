use board::*;
use point2::*;

pub fn drop_jewels(board: &mut Board) -> bool {
    let mut dropped = false;
    for x in 0..board.width() {
        for y in 0..board.height() - 1 {
            if board[x][y].is_none() && board[x][y + 1].is_some() {
                board[x][y] = board[x][y + 1];
                board[x][y + 1] = None;
                dropped = true;
            }
        }
    }
    dropped
}

#[cfg(test)]
#[test]
fn test_drop() {
    use columns::Jewel;
    let mut board = Board::new(1, 3);
    board[0][2] = Some(Jewel::Red);
    drop_jewels(&mut board);
    assert!(board[0][2].is_none());
    assert!(board[0][1] == Some(Jewel::Red));
}
