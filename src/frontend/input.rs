use std::char;

use ncurses::*;

pub static ESCAPE_CODE: i32 = 27;
pub static KEY_LEFT_SEQ: [i32; 3] = [27, 91, 68];
pub static KEY_RIGHT_SEQ: [i32; 3] = [27, 91, 67];
pub static KEY_HOME_SEQ: [i32; 3] = [27, 91, 72];
pub static KEY_END_SEQ: [i32; 3] = [27, 91, 70];
pub static KEY_BACKSPACE_SEQ: [i32; 1] = [127];

pub enum Key {
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Tab,
    Backspace,
    Delete,
    Char(char),
    Escape,
    Other
}

pub enum Modifier {
    Alt(i32)
}

pub enum Input {
    Kb(Key, Option<Modifier>),
    Resize,
    None
}

pub fn read_key() -> (Input, i32) {
    let key = wgetch(stdscr);

    let input = match key {
        ERR => Input::None,
        KEY_RESIZE => Input::Resize,
        KEY_LEFT => Input::Kb(Key::Left, None),
        KEY_RIGHT => Input::Kb(Key::Right, None),
        KEY_UP => Input::Kb(Key::Up, None),
        KEY_DOWN => Input::Kb(Key::Down, None),
        KEY_HOME => Input::Kb(Key::Home, None),
        KEY_PPAGE => Input::Kb(Key::PageUp, None),
        KEY_NPAGE => Input::Kb(Key::PageDown, None),
        KEY_END => Input::Kb(Key::End, None),
        KEY_DC => Input::Kb(Key::Delete, None),
        KEY_BACKSPACE => Input::Kb(Key::Backspace, None),
        KEY_BTAB => Input::Kb(Key::Tab, None),
        value => parse_key_code(value)
    };
    (input, key)
}

fn parse_key_code(code: i32) -> Input {
    let mut modifier = None;
    let mut pending = code;

    if pending == ESCAPE_CODE {
        let new_code = wgetch(stdscr);
        if new_code == ERR {
            return Input::Kb(Key::Escape, None)
        }
        pending = new_code;
        modifier = Some(Modifier::Alt(pending));
    }

    match char::from_u32(pending as u32) {
        Some(parsed_code) => {
            Input::Kb(Key::Char(parsed_code), modifier)
        },
        _ => Input::Kb(Key::Other, None)
    }
}
