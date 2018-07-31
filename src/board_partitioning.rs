use point2::*;

pub fn get_columns(board_size: &Size2) -> Vec<Vec<P2>> {
    let mut columns = Vec::with_capacity(board_size.width());
    for x in 0..board_size.width() {
        let mut ps: Vec<P2> = vec![];
        for y in 0..board_size.height() {
            ps.push(P2 { x, y });
        }
        columns.push(ps);
    }
    columns
}

pub fn get_rows(board_size: &Size2) -> Vec<Vec<P2>> {
    let mut rows = Vec::with_capacity(board_size.height());
    for y in 0..board_size.height() {
        let mut ps: Vec<P2> = vec![];
        for x in 0..board_size.width() {
            ps.push(P2 { x, y });
        }
        rows.push(ps);
    }
    rows
}

pub fn get_upwards_diagonals(board_size: &Size2) -> Vec<Vec<P2>> {
    let mut diagonals: Vec<Vec<P2>> = vec![];
    for x in 0..board_size.width() {
        let mut xs: Vec<P2> = vec![];
        let mut y = 0;
        for h in x..board_size.width() {
            if y < board_size.height() {
                xs.push(P2::new(h, y));
            }
            y += 1;
        }
        diagonals.push(xs);
    }
    for y in 1..board_size.height() {
        let mut ys: Vec<P2> = vec![];
        let mut x = 0;
        for v in y..board_size.height() {
            if x < board_size.width() {
                ys.push(P2::new(x, v));
            }
            x += 1;
        }
        diagonals.push(ys);
    }
    diagonals
}

pub fn get_downwards_diagonals(board_size: &Size2) -> Vec<Vec<P2>> {
    let mut diagonals: Vec<Vec<P2>> = vec![];
    for x in 0..board_size.width() {
        let mut xs: Vec<P2> = vec![];
        let mut y = board_size.height() - 1;
        for h in x..board_size.width() {
            xs.push(P2::new(h, y));
            if y == 0 {
                break;
            }
            y -= 1;
        }
        diagonals.push(xs);
    }

    for y in 0..board_size.height() - 1 {
        let mut ys: Vec<P2> = vec![];
        let mut x = 0;
        let mut v = y;
        loop {
            ys.push(P2::new(x, v));
            x += 1;
            if v == 0 || x == board_size.width() {
                break;
            }
            v -= 1;
        }
        diagonals.push(ys);
    }
    diagonals
}

pub fn get_all_lines<T: Size2>(board_size: &T) -> Vec<Vec<P2>> {
    let mut all: Vec<Vec<Vec<P2>>> = vec![
        get_columns(board_size),
        get_rows(board_size),
        get_downwards_diagonals(board_size),
        get_upwards_diagonals(board_size),
    ];
    let mut out = vec![];
    for each in &mut all {
        out.append(each);
    }
    out
}

#[cfg(test)]
#[test]
fn test_columns_1() {
    use board::Board;
    let board: Board = Board::new(7, 13);
    let cs = get_columns(&board);
    assert!(cs.len() == board.width());
    for c in cs {
        assert!(c.len() == board.height());
    }
}

#[cfg(test)]
#[test]
fn test_rows_1() {
    use board::Board;
    let board: Board = Board::new(7, 13);
    let rs = get_rows(&board);
    assert!(rs.len() == board.height());
    for r in rs {
        assert!(r.len() == board.width());
    }
}
