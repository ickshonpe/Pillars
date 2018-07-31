use input::InputState;
use graphics;
pub trait GameState {
    fn update(self: Box<Self>, time_delta: f64, input_state: &InputState) -> Box<GameState>;

    fn draw(&self, ctx: &graphics::GraphicsContext);
}
