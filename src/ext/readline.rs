use libc::{c_int, c_char, FILE};

pub type RlCommandFuncT = Option<extern "C" fn(c_int, c_int) -> c_int>;
pub type RlVcpfuncT = Option<extern "C" fn(*mut c_char)>;
pub type RlVoidfuncT = Option<extern "C" fn()>;
pub type RlVintfuncT = Option<extern "C" fn(c_int)>;
pub type RlGetcFuncT = Option<extern "C" fn(*mut FILE) -> c_int>;
pub type RlHookFuncT = Option<extern "C" fn() -> c_int>;

#[link(name = "readline")]
extern "C" {
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
    pub fn rl_reset_line_state();
    pub fn rl_callback_handler_install(prompt: *const c_char, callback: RlVcpfuncT);
    pub fn rl_callback_handler_remove();
}
