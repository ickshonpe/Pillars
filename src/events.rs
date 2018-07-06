use std;
use sdl2;
use sdl2::event::Event;
use input::InputState;
use input::Buttons;
use sdl2::keyboard::Keycode;

pub fn process_sdl_event(
    event: sdl2::event::Event,
    input_state: &mut InputState,
    key_bindings: &std::collections::HashMap<Keycode, Buttons> ) {
    match event {
        Event::Quit { .. } => {
            input_state.press(Buttons::Quit);
        },
        Event::KeyDown { keycode, .. } => {
            if let Some(keycode) = keycode {
                if let Some(&button) = key_bindings.get(&keycode) {
                    input_state.press(button);
                }
            }
        },
        Event::KeyUp { keycode, .. } => {
            if let Some(keycode) = keycode {
                if let Some(&button) = key_bindings.get(&keycode) {
                    input_state.release(button);
                }
            }
        },
        _ => {}
    }
}