use input::Buttons;
use input::InputState;
use sdl2;
use sdl2::controller;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std;

pub fn process_sdl_event(
    event: &sdl2::event::Event,
    input_state: &mut InputState,
    key_bindings: &std::collections::HashMap<Keycode, Buttons>,
    controller_bindings: &std::collections::HashMap<controller::Button, Buttons>,
    controllers: &mut Vec<sdl2::controller::GameController>,
    controller_subsystem: &sdl2::GameControllerSubsystem,
) {
    match event {
        Event::Quit { .. } => {
            input_state.press(Buttons::Quit);
        }
        Event::KeyDown {
            keycode, repeat, ..
        } => {
            if !repeat {
                if let Some(keycode) = keycode {
                    if let Some(&button) = key_bindings.get(&keycode) {
                        input_state.press(button);
                    }
                }
            }
        }
        Event::KeyUp {
            keycode, repeat, ..
        } => {
            if !repeat {
                if let Some(keycode) = keycode {
                    if let Some(&button) = key_bindings.get(&keycode) {
                        input_state.release(button);
                    }
                }
            }
        }
        Event::ControllerDeviceAdded { which, .. } => {
            if let Ok(c) = controller_subsystem.open(*which) {
                if !controllers
                    .iter()
                    .any(|d| c.instance_id() == d.instance_id())
                {
                    controllers.push(c);
                }
            }
        }
        Event::ControllerDeviceRemoved { which, .. } => {
            controllers.retain(|c| c.instance_id() != *which);
        }

        Event::ControllerButtonDown {
            timestamp,
            which,
            button,
        } => {
            if let Some(&button) = controller_bindings.get(&button) {
                input_state.press(button);
            }
        }
        Event::ControllerButtonUp {
            timestamp,
            which,
            button,
        } => {
            if let Some(&button) = controller_bindings.get(&button) {
                input_state.release(button);
            }
        }
        Event::ControllerAxisMotion { axis, value, .. } => {
            if *axis == sdl2::controller::Axis::LeftX {
                let dead_zone = 13_000;
                if *value < -dead_zone {
                    input_state.press(Buttons::Left);
                    input_state.release(Buttons::Right);
                } else if *value < dead_zone {
                    input_state.release(Buttons::Left);
                    input_state.release(Buttons::Right);
                } else {
                    input_state.press(Buttons::Right);
                    input_state.release(Buttons::Left);
                }
            }
            if *axis == sdl2::controller::Axis::LeftY {
                let dead_zone = 13_000;
                if dead_zone < *value {
                    input_state.press(Buttons::Down);
                } else {
                    input_state.release(Buttons::Down);
                }
            }
        }
        _ => {}
    }
}
