extern crate notify;
extern crate ncurses;
extern crate getopts;
extern crate toml;
extern crate rustc_serialize;
extern crate regex;

use std::env;

use settings::SettingsBuilder;

pub mod settings;
pub mod runner;
pub mod flow;
pub mod ui;

fn main() {
    let args = env::args().collect();

    let settings = SettingsBuilder::new(args).construct();
    runner::execute(settings);
}
