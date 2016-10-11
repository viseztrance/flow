//
// The following code is based on
// https://github.com/ulfalizer/readline-and-ncurses by Ulf Magnusson
//

use libc::{FILE, free, c_void, c_char};
use std::ffi::CStr;
use ncurses::*;
use unicode_width::UnicodeWidthStr;
use unicode_segmentation::UnicodeSegmentation;

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

pub fn read_buffer<'a>() -> &'a str {
    unsafe {
        cstr_ptr_to_str(rl_line_buffer)
    }
}

pub fn read_prompt<'a>() -> &'a str {
    unsafe {
        cstr_ptr_to_str(rl_display_prompt)
    }
}

pub fn move_cursor() {
    unsafe {
        let window = command_window.unwrap();
        let prompt = read_prompt();
        let buffer = read_buffer();

        let mut current_bytes = 0;
        let cursor_position = buffer
            .graphemes(true)
            .take_while(|x| {
                current_bytes += x.len();
                current_bytes <= rl_point as usize
            })
            .collect::<String>()
            .width();
        curs_set(CURSOR_VISIBILITY::CURSOR_VERY_VISIBLE);
        wmove(window, 0, (prompt.len() + 1 + cursor_position) as i32);
        wrefresh(window);
    }
}

pub fn reset() {
    unsafe {
        rl_reset_line_state();
    }
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

pub extern "C" fn handle_redisplay() {
    let window = unsafe { command_window.unwrap() };
    let prompt = read_prompt();
    let buffer = read_buffer();

    werase(window);

    if buffer.is_empty() {
        wprintw(window, prompt);
    } else {
        wprintw(window, &format!("{} {}", prompt, buffer));
    }

    move_cursor();
}

extern "C" fn handle_input(line_ptr: *mut c_char) {
    if line_ptr.is_null() {
        return;
    }

    unsafe { free(line_ptr as *mut c_void); }
}

unsafe fn cstr_ptr_to_str<'a>(c_str: *const i8) -> &'a str {
    CStr::from_ptr(c_str).to_str().unwrap()
}
