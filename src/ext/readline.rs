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

use libc::{c_int, c_char, FILE};

pub const RL_STATE_NONE: i32 = 0x000000;
pub const RL_STATE_INITIALIZING: i32 = 0x000001;
pub const RL_STATE_INITIALIZED: i32 = 0x000002;
pub const RL_STATE_TERMPREPPED: i32 = 0x000004;
pub const RL_STATE_READCMD: i32 = 0x000008;
pub const RL_STATE_METANEXT: i32 = 0x000010;
pub const RL_STATE_DISPATCHING: i32 = 0x000020;
pub const RL_STATE_MOREINPUT: i32 = 0x000040;
pub const RL_STATE_ISEARCH: i32 = 0x000080;
pub const RL_STATE_NSEARCH: i32 = 0x000100;
pub const RL_STATE_SEARCH: i32 = 0x000200;
pub const RL_STATE_NUMERICARG: i32 = 0x000400;
pub const RL_STATE_MACROINPUT: i32 = 0x000800;
pub const RL_STATE_MACRODEF: i32 = 0x001000;
pub const RL_STATE_OVERWRITE: i32 = 0x002000;
pub const RL_STATE_COMPLETING: i32 = 0x004000;
pub const RL_STATE_SIGHANDLER: i32 = 0x008000;
pub const RL_STATE_UNDOING: i32 = 0x010000;
pub const RL_STATE_INPUTPENDING: i32 = 0x020000;
pub const RL_STATE_TTYCSAVED: i32 = 0x040000;
pub const RL_STATE_CALLBACK: i32 = 0x080000;
pub const RL_STATE_VIMOTION: i32 = 0x100000;
pub const RL_STATE_MULTIKEY: i32 = 0x200000;
pub const RL_STATE_VICMDONCE: i32 = 0x400000;
pub const RL_STATE_DONE: i32 = 0x800000;

pub type RlCommandFuncT = Option<extern "C" fn(c_int, c_int) -> c_int>;
pub type RlVcpfuncT = Option<extern "C" fn(*mut c_char)>;
pub type RlVoidfuncT = Option<extern "C" fn()>;
pub type RlVintfuncT = Option<extern "C" fn(c_int)>;
pub type RlGetcFuncT = Option<extern "C" fn(*mut FILE) -> c_int>;
pub type RlHookFuncT = Option<extern "C" fn() -> c_int>;

#[link(name = "readline")]
extern "C" {
    pub static mut rl_readline_state: c_int;
    pub static mut rl_display_prompt: *mut c_char;
    pub static mut rl_line_buffer: *mut c_char;
    pub static mut rl_point: c_int;
    pub static mut rl_change_environment: c_int;
    pub static mut rl_catch_signals: c_int;
    pub static mut rl_catch_sigwinch: c_int;
    pub static mut rl_getc_function: RlGetcFuncT;
    pub static mut rl_deprep_term_function: RlVoidfuncT;
    pub static mut rl_prep_term_function: RlVintfuncT;
    pub static mut rl_input_available_hook: RlHookFuncT;
    pub static mut rl_redisplay_hook: RlVoidfuncT;
    pub static mut rl_redisplay_function: RlVoidfuncT;

    pub fn rl_bind_key(key: c_int, callback: RlCommandFuncT) -> c_int;
    pub fn rl_unbind_key(key: c_int) -> c_int;
    pub fn rl_callback_read_char();
    pub fn rl_insert(_: c_int, _: c_int) -> c_int;
    pub fn rl_callback_handler_install(prompt: *const c_char, callback: RlVcpfuncT);
    pub fn rl_callback_handler_remove();
}


pub mod history {
    use libc::{c_int, c_char};

    #[link(name = "readline")]
    extern "C" {
        pub static mut history_length: c_int;

        pub fn using_history();
        pub fn add_history(input: *const c_char);
        pub fn read_history(filename: *const c_char) -> c_int;
        pub fn write_history(filename: *const c_char) -> c_int;
        pub fn history_truncate_file(filename: *const c_char, nlines: c_int) -> c_int;
        pub fn history_set_pos(pos: c_int) -> c_int;
    }
}
