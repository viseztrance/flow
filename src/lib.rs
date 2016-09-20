#[macro_use] extern crate lazy_static;

extern crate libc;
extern crate notify;
extern crate ncurses;
extern crate getopts;
extern crate toml;
extern crate rustc_serialize;
extern crate regex;

pub mod ext;
pub mod settings;
pub mod runner;
pub mod utils;
pub mod core;
pub mod frontend;
