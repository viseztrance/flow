use std::char;

use ncurses::*;

pub enum Key {
    Left,
    Right,
    Up,
    Down,
    MouseWheelUp,
    MouseWheelDown,
    Resize,
    Char(char),
    Other,
    None
}

pub fn read_key() -> Key {
    match getch() {
        ERR => Key::None,
        KEY_LEFT => Key::Left,
        KEY_RIGHT => Key::Right,
        KEY_UP => Key::Up,
        KEY_DOWN => Key::Down,
        KEY_RESIZE => Key::Resize,
        KEY_MOUSE => key_from_mouse_event(),
        value => {
            // TODO: use `keyname` to determine special keys
            match char::from_u32(value as u32) {
                Some(parsed_value) => Key::Char(parsed_value),
                _ => Key::Other
            }
        }
    }
}

fn key_from_mouse_event() -> Key {
    let ref mut event = MEVENT {
        id: 0, x: 0, y: 0, z: 0, bstate: 0
    };
    if getmouse(event) == OK {
        if (event.bstate & BUTTON4_PRESSED as u64) != 0 {
            return Key::MouseWheelUp
        } else if (event.bstate & BUTTON5_PRESSED as u64) != 0 {
            return Key::MouseWheelDown
        }
    }
    Key::Other
}
