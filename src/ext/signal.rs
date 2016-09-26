pub static SIGINT: u32 = 2;
pub static SIGQUIT: u32 = 3;

extern "C" {
    pub fn raise(sig: u32) -> u32;
    pub fn signal(signum: u32, sighandler_t: extern fn(u32)) -> extern fn(u32);
}
