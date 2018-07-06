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
        Event::AppTerminating { .. } => {},
        Event::AppLowMemory { .. } => {},
        Event::AppWillEnterBackground { .. } => {},
        Event::AppDidEnterBackground { .. } => {},
        Event::AppWillEnterForeground { .. } => {},
        Event::AppDidEnterForeground { .. } => {},
        Event::Window { .. } => {},
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
        Event::TextEditing { .. } => {},
        Event::TextInput { .. } => {},
        Event::MouseMotion { .. } => {},
        Event::MouseButtonDown { .. } => {},
        Event::MouseButtonUp { .. } => {},
        Event::MouseWheel { .. } => {},
        Event::JoyAxisMotion { .. } => {},
        Event::JoyBallMotion { .. } => {},
        Event::JoyHatMotion { .. } => {},
        Event::JoyButtonDown { .. } => {},
        Event::JoyButtonUp { .. } => {},
        Event::JoyDeviceAdded { .. } => {},
        Event::JoyDeviceRemoved { .. } => {},
        Event::ControllerAxisMotion { .. } => {},
        Event::ControllerButtonDown { .. } => {},
        Event::ControllerButtonUp { .. } => {},
        Event::ControllerDeviceAdded { .. } => {},
        Event::ControllerDeviceRemoved { .. } => {},
        Event::ControllerDeviceRemapped { .. } => {},
        Event::FingerDown { .. } => {},
        Event::FingerUp { .. } => {},
        Event::FingerMotion { .. } => {},
        Event::DollarGesture { .. } => {},
        Event::DollarRecord { .. } => {},
        Event::MultiGesture { .. } => {},
        Event::ClipboardUpdate { .. } => {},
        Event::DropFile { .. } => {},
        Event::User { .. } => {},
        Event::Unknown { .. } => {},
    }

}