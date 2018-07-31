#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct P2 {
    pub x: usize,
    pub y: usize,
}

pub trait Size2 {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

impl P2 {
    pub fn new(x: usize, y: usize) -> Self {
        P2 { x, y }
    }
    pub fn trans_up(self) -> Self {
        P2 {
            x: self.x,
            y: self.y + 1,
        }
    }
    pub fn trans_down(self) -> Self {
        P2 {
            x: self.x,
            y: self.y - 1,
        }
    }
    pub fn trans_left(self) -> Self {
        P2 {
            x: self.x - 1,
            y: self.y,
        }
    }
    pub fn trans_right(self) -> Self {
        P2 {
            x: self.x + 1,
            y: self.y,
        }
    }
    pub fn up(&mut self) -> &mut Self {
        self.y += 1;
        self
    }
    pub fn down(&mut self) -> &mut Self {
        self.y -= 1;
        self
    }
    pub fn left(&mut self) -> &mut Self {
        self.x -= 1;
        self
    }
    pub fn right(&mut self) -> &mut Self {
        self.x += 1;
        self
    }
    pub fn zero() -> Self {
        P2 { x: 0, y: 0 }
    }
}

impl Default for P2 {
    fn default() -> Self {
        P2 { x: 0, y: 0 }
    }
}

#[cfg(test)]
#[test]
fn test_right() {
    let mut p = P2::zero();
    p.right().right();
    assert!(p.x == 2);
}

#[cfg(test)]
#[test]
fn test_left() {
    let mut p = P2::zero();
    p.right().right();
    p.left().left();
    assert!(p.x == 0);
    assert!(p == P2::zero());
}
