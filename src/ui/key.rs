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
        value => {
            // TODO: use `keyname` to determine special keys
            match char::from_u32(value as u32) {
                Some(parsed_value) => Key::Char(parsed_value),
                _ => Key::Other
            }
        }
    }
}
