use std::char;

use ncurses::*;

static ESCAPE_OR_ALT_MODIFIER: i32 = 27;

pub enum Key {
    Left,
    Right,
    Up,
    Down,
    Char(char),
    Escape,
    Other
}

pub enum Modifier {
    Ctrl,
    Alt
}

pub enum Input {
    Kb(Key, Option<Modifier>),
    Resize,
    None
}



pub fn read_key() -> Input {
    match getch() {
        ERR => Input::None,
        KEY_RESIZE => Input::Resize,
        KEY_LEFT => Input::Kb(Key::Left, None),
        KEY_RIGHT => Input::Kb(Key::Right, None),
        KEY_UP => Input::Kb(Key::Up, None),
        KEY_DOWN => Input::Kb(Key::Down, None),
        value => parse_key_value(value)
    }
}

fn parse_key_value(mut value: i32) -> Input {
    let mut modifier = None;

    if value == ESCAPE_OR_ALT_MODIFIER {
        let new_value = getch();
        if new_value == ERR {
            return Input::Kb(Key::Escape, None)
        }
        value = new_value;
        modifier = Some(Modifier::Alt);
    }

    match char::from_u32(value as u32) {
        Some(parsed_value) => Input::Kb(Key::Char(parsed_value), modifier),
        _ => Input::Kb(Key::Other, None)
    }
}
