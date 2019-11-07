use input::InputState;
use graphics;
pub trait GameState {
    fn update(self: Box<Self>, time_delta: f64, input_state: &InputState) -> Box<dyn GameState>;

    fn draw(&self, ctx: &graphics::GraphicsContext);
}
