use graphics::Vector2;

#[derive(Copy, Clone, Debug)]
pub struct Rectangle {
    pub position: Vector2,
    pub size: Vector2,
}

impl Rectangle {
    pub fn left(&self) -> f32 {
        self.position[0]
    }
    pub fn right(&self) -> f32 {
        self.position[0] + self.size[0]
    }
    pub fn bottom(&self) -> f32 {
        self.position[0]
    }
    pub fn top(&self) -> f32 {
        self.position[1] + self.size[1]
    }
}