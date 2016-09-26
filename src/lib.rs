#[macro_use] extern crate lazy_static;

extern crate libc;
extern crate regex;
extern crate toml;
extern crate rustc_serialize;
extern crate getopts;
extern crate unicode_width;
extern crate unicode_segmentation;
extern crate ncurses;

#[macro_use] pub mod macros;
pub mod ext;
pub mod utils;
pub mod core;
pub mod frontend;
