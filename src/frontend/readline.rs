//
// The following code is based on these implementations:
// https://github.com/ulfalizer/readline-and-ncurses by Ulf Magnusson
// https://github.com/postmodern/ncurses-readline-rs by Postmodern
//

use libc::{FILE, free, c_void, c_char};
use std::ffi::CStr;
use ncurses::*;

use ext::readline::*;

static mut input: i32 = 0;
static mut input_available: bool = false;
static mut command_window: Option<WINDOW> = None;

pub fn init() {
    unsafe {
        rl_change_environment = 0; // Conflicts with ncurses
        rl_catch_signals = 0;
        rl_catch_sigwinch = 0;
        rl_deprep_term_function = None;
        rl_prep_term_function = None;

        rl_getc_function = Some(getc);
        rl_input_available_hook = Some(is_input_available);
        rl_redisplay_function = Some(handle_redisplay);
    }
}

pub fn render(prompt: &str, window: WINDOW) {
    unsafe {
        command_window = Some(window);

        rl_callback_handler_install(prompt.as_ptr() as (*const i8), Some(handle_input));
    }
}

pub fn forward_input(key: i32) {
    unsafe {
        input = key;
        input_available = true;
        rl_callback_read_char();
    }
}

pub fn terminate() {
    unsafe {
        rl_callback_handler_remove();
    }
}

fn read_prompt<'a>() -> &'a str {
    unsafe {
        cstr_ptr_to_str(rl_display_prompt)
    }
}

pub fn read_buffer<'a>() -> &'a str {
    unsafe {
        cstr_ptr_to_str(rl_line_buffer)
    }
}

pub fn update_buffer(value: &str) {
    unsafe {
        rl_replace_line(value.as_ptr() as *mut i8, 0);
    }
}

extern "C" fn getc(_: *mut FILE) -> i32 {
    unsafe {
        input_available = false;

        input
    }
}

extern "C" fn is_input_available() -> i32 {
    unsafe {
        input_available as i32
    }
}

pub extern "C" fn handle_redisplay() {
    unsafe {
        let window = command_window.unwrap();

        werase(window);

        mvwprintw(window, 0, 0, &format!("{} {}", read_prompt(), read_buffer()));

        let cursor_position =
            cstr_ptr_to_str(rl_display_prompt).len() as i32 +
            rl_point + 1;

        wmove(window, 0, cursor_position);

        wrefresh(window);
    }
}

extern "C" fn handle_input(line_ptr: *mut c_char) {
    if line_ptr.is_null() {
        return;
    }

    let line = unsafe { cstr_ptr_to_str(line_ptr) };
    handle_redisplay();

    if !line.is_empty() {
        // add history
        handle_redisplay();
    }

    unsafe { free(line_ptr as *mut c_void); }
}

unsafe fn cstr_ptr_to_str<'a>(c_str: *const i8) -> &'a str {
    CStr::from_ptr(c_str).to_str().unwrap()
}
