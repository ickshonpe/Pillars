use columns::*;
use point2::*;
use std::ops::{Index, IndexMut};

impl Size2 for Board {
    fn width(&self) -> usize {
        self.data.len()
    }
    fn height(&self) -> usize {
        self.data[0].len()
    }
}

impl Index<P2> for Board {
    type Output = Option<Jewel>;
    fn index(&self, p: P2) -> &Self::Output {
        &self.data[p.x as usize][p.y as usize]
    }
}

impl Index<usize> for Board {
    type Output = Vec<Option<Jewel>>;
    fn index(&self, x: usize) -> &Self::Output {
        &self.data[x]
    }
}

impl IndexMut<P2> for Board {
    fn index_mut(&mut self, index: P2) -> &mut Option<Jewel> {
        &mut self.data[index.x][index.y]
    }
}

impl IndexMut<usize> for Board {
    fn index_mut(&mut self, x: usize) -> &mut Vec<Option<Jewel>> {
        &mut self.data[x]
    }
}

pub struct Board {
    data: Vec<Vec<Option<Jewel>>>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Board {
        Board {
            data: vec![vec![None; height]; width],
        }
    }
}
