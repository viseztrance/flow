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

use std::env;
use libc::{FILE, free, c_void, c_char};
use std::ffi::{CStr, CString};
use std::cmp::max;

use ncurses::*;
use unicode_width::UnicodeWidthStr;
use unicode_segmentation::UnicodeSegmentation;

use ext::readline::*;

const HISTORY_FILENAME: &'static str = ".flow_history";
const MAX_HISTORY_LINES: i32 = 1000;
const MIN_HISTORY_LINE_WIDTH: usize = 2;

#[allow(non_upper_case_globals)]
static mut input: i32 = 0;

#[allow(non_upper_case_globals)]
static mut input_available: bool = false;

#[allow(non_upper_case_globals)]
static mut command_window: Option<WINDOW> = None;

#[allow(non_upper_case_globals)]
static mut last_history_item: &'static str = "\0";

pub fn init() {
    unsafe {
        rl_change_environment = 0; // Conflicts with ncurses
        rl_catch_signals = 0;
        rl_catch_sigwinch = 0;
        rl_deprep_term_function = None;
        rl_prep_term_function = None;

        rl_unbind_key('\r' as i32); // Unbind Enter
        rl_unbind_key('\n' as i32); // Unbind Control + J
        rl_unbind_key('\t' as i32); // Unbind Tab
        rl_unbind_key('L' as i32 - '@' as i32); // Unbind Control + L
        rl_getc_function = Some(getc);
        rl_input_available_hook = Some(is_input_available);
        rl_redisplay_function = Some(handle_redisplay);
    }
}

pub fn render(prompt: &str, window: WINDOW) {
    unsafe {
        command_window = Some(window);
        let prompt_cstring = CString::new(prompt).unwrap();

        rl_callback_handler_install(prompt_cstring.as_ptr(), Some(handle_input));
    }
}

pub fn forward_input(key: i32) {
    unsafe {
        input = key;
        input_available = true;
        rl_callback_read_char();
    }
}

pub fn use_history() {
    unsafe {
        history::using_history();
    }
}

pub fn add_history() {
    let buffer = read_buffer();

    unsafe {
        if last_history_item != buffer && buffer.width() > MIN_HISTORY_LINE_WIDTH {
            history::add_history(rl_line_buffer);
            last_history_item = buffer;
            history::history_set_pos(history::history_length);
        }
    }
}

pub fn read_history() {
    unsafe {
        history::read_history(history_file_path());
    }
}

pub fn write_history() {
    let path = history_file_path();

    unsafe {
        history::write_history(path);
        history::history_truncate_file(path, MAX_HISTORY_LINES);
    }
}

pub fn is_history() -> bool {
    unsafe {
        rl_readline_state & RL_STATE_ISEARCH > 0 || rl_readline_state & RL_STATE_NSEARCH > 0 ||
        rl_readline_state & RL_STATE_SEARCH > 0
    }
}

fn history_file_path() -> *const i8 {
    let mut path = env::home_dir().unwrap();
    path.push(HISTORY_FILENAME);
    let path_cstring = CString::new(path.to_str().unwrap()).unwrap();

    path_cstring.as_ptr()
}

pub fn terminate() {
    unsafe {
        rl_callback_handler_remove();
    }
}

pub fn read_buffer<'a>() -> &'a str {
    unsafe { cstr_ptr_to_str(rl_line_buffer) }
}

pub fn read_prompt<'a>() -> &'a str {
    unsafe { cstr_ptr_to_str(rl_display_prompt) }
}

fn read_cursor_position() -> i32 {
    unsafe {
        let prompt = read_prompt();
        let buffer = read_buffer();

        let mut current_bytes = 0;
        let cursor_position = buffer.graphemes(true)
            .take_while(|x| {
                current_bytes += x.len();
                current_bytes <= rl_point as usize
            })
            .collect::<String>()
            .width();
        (prompt.width() + 1 + cursor_position) as i32
    }
}

fn wrapping_offset(cursor_position: i32) -> i32 {
    let window = unsafe { command_window.unwrap() };
    let mut x = 0;
    let mut y = 0;
    getmaxyx(window, &mut y, &mut x);

    max(cursor_position - x + 1, 0)
}

pub fn move_cursor() {
    let window = unsafe { command_window.unwrap() };
    let cursor_position = read_cursor_position();
    curs_set(CURSOR_VISIBILITY::CURSOR_VERY_VISIBLE);
    wmove(window,
          0,
          cursor_position - wrapping_offset(cursor_position));
    wrefresh(window);
}

pub extern "C" fn handle_redisplay() {
    let window = unsafe { command_window.unwrap() };
    let prompt = read_prompt();
    let buffer = read_buffer();

    werase(window);

    if buffer.is_empty() {
        wprintw(window, prompt);
    } else {
        let clipped_buffer = buffer.graphemes(true)
            .skip(wrapping_offset(read_cursor_position()) as usize)
            .collect::<String>();
        wprintw(window, &format!("{} {}", prompt, clipped_buffer));
    }

    move_cursor();
}

extern "C" fn getc(_: *mut FILE) -> i32 {
    unsafe {
        input_available = false;
        input
    }
}

extern "C" fn is_input_available() -> i32 {
    unsafe { input_available as i32 }
}

extern "C" fn handle_input(line_ptr: *mut c_char) {
    if line_ptr.is_null() {
        return;
    }

    unsafe {
        free(line_ptr as *mut c_void);
    }
}

unsafe fn cstr_ptr_to_str<'a>(c_str: *const i8) -> &'a str {
    CStr::from_ptr(c_str).to_str().unwrap()
}
