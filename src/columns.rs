use point2::P2;
use std::ops::IndexMut;
use std::ops::Index;
use random;
use graphics;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Jewel {
    Red,
    Yellow,
    Blue,
    Green,
    Orange,
}

impl Jewel {
    pub fn color_gl(self) -> graphics::Color {
        match self {
            Jewel::Red => { graphics::RED }
            Jewel::Yellow => { graphics::YELLOW }
            Jewel::Green => { graphics::GREEN }
            Jewel::Blue => { graphics::BLUE }
            Jewel::Orange => { graphics::ORANGE }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Column {
    pub jewels: [Jewel; 3],
    pub position: P2
}

impl Index<usize> for Column {
    type Output = Jewel;
    fn index(&self, index: usize) -> &Self::Output {
        &self.jewels[index]
    }
}

impl IndexMut<usize> for Column {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.jewels[index]
    }
}

impl Column {
    pub fn new(position: P2) -> Self {
        Column {
            jewels: [select_random_jewel(), select_random_jewel(), select_random_jewel()],
            position
        }
    }

    pub fn cycle_up(&mut self) {
        let j0 = self[0];
        let j1 = self[1];
        let j2 = self[2];
        self[0] = j2;
        self[1] = j0;
        self[2] = j1;
    }

    pub fn cycle_down(&mut self) {
        let j0 = self[0];
        let j1 = self[1];
        let j2 = self[2];
        self[0] = j1;
        self[1] = j2;
        self[2] = j0;
    }
}

impl Jewel {
    pub fn all_jewels() -> [Jewel; 5] {
        [Jewel::Red, Jewel::Orange, Jewel::Green, Jewel::Blue, Jewel::Yellow]
    }
}

pub fn select_random_jewel() -> Jewel {
    random::select_random(&Jewel::all_jewels())
}
