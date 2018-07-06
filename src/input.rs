use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Buttons {
    Left,
    Right,
    Down,
    CycleUp,
    CycleDown,
    Quit
}

#[derive(Clone, Default)]
struct InputData {
    buttons: HashSet<Buttons>
}

impl InputData {
    fn down(&self, button: Buttons) -> bool {
        self.buttons.contains(&button)
    }
    fn up(&self, button: Buttons) -> bool {
        !self.down(button)
    }
    fn press(&mut self, button: Buttons) {
        self.buttons.insert(button);
    }
    fn release(&mut self, button: Buttons) {
        self.buttons.remove(&button);
    }
}

#[derive(Default)]
pub struct InputState {
    current: InputData,
    previous: InputData
}

impl InputState {
    pub fn down(&self, button: Buttons) -> bool {
        self.current.down(button)
    }
    pub fn up(&self, button: Buttons) -> bool {
        self.current.up(button)
    }
    pub fn just_pressed(&self, button: Buttons) -> bool {
        self.down(button)
            && self.previous.up(button)
    }
    pub fn just_released(&self, button: Buttons) -> bool {
        self.up(button)
            && self.previous.down(button)
    }
    pub fn changed(&self, button: Buttons) -> bool {
        self.down(button) != self.previous.down(button)
    }
    pub fn store_current(&mut self) {
        self.previous = self.current.clone()
    }
    pub fn press(&mut self, button: Buttons) {
        self.current.press(button);
    }
    pub fn release(&mut self, button: Buttons) {
        self.current.release(button);
    }
}
