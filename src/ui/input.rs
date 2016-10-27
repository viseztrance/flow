/**
 * Flow - Realtime log analyzer
 * Copyright (C) 2016 Daniel Mircea
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

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
    Other,
}

pub enum Modifier {
    Alt(i32),
    Ctrl,
}

pub enum Input {
    Kb(Key, Option<Modifier>),
    Resize,
    None,
}

pub fn read_key() -> (Input, i32) {
    let key = wgetch(stdscr());

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
        value => parse_key_code(value),
    };
    (input, key)
}

fn parse_key_code(code: i32) -> Input {
    let mut modifier = None;
    let mut pending = code;

    if pending == ESCAPE_CODE {
        let new_code = wgetch(stdscr());
        if new_code == ERR {
            return Input::Kb(Key::Escape, None);
        }
        pending = new_code;
        modifier = Some(Modifier::Alt(pending));
    } else {
        let name = keyname(pending);
        if name.contains('^') {
            modifier = Some(Modifier::Ctrl);
            let value = Key::Char(name.chars().last().unwrap());
            return Input::Kb(value, modifier);
        }
    }

    match char::from_u32(pending as u32) {
        Some(parsed_code) => Input::Kb(Key::Char(parsed_code), modifier),
        _ => Input::Kb(Key::Other, None),
    }
}
