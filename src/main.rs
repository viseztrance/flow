extern crate notify;
extern crate ncurses;
extern crate getopts;
extern crate toml;
extern crate rustc_serialize;

use std::env;

use std::sync::{Arc, Mutex};
use std::thread;

use settings::{SettingsBuilder, Settings};
use flow::tail::Tail;
use flow::flow::Flow;

pub mod settings;
pub mod flow {
    pub mod flow;
    pub mod tail;
    pub mod buffer;
    pub mod ui;
}

fn main() {
    let args = env::args().collect();

    let settings = SettingsBuilder::new(args).make();
    run(settings);
}

fn run(settings: Settings) {
    let mut tail = Tail::new(settings.path_to_target_file.clone());

    let lines = Arc::new(Mutex::new(tail.read_lines(settings.last_lines_count)));

    let reader_lines = lines.clone();
    let reader_thread = thread::spawn(move || {
        tail.watch(|data| reader_lines.lock().unwrap().extend(data));
    });

    let consumer_lines = lines.clone();
    let consumer_thread = thread::spawn(move || {
        let mut flow = Flow::new(settings);
        flow.render();
        flow.process(consumer_lines);
        flow.terminate();
    });

    let _ = reader_thread.join();
    let _ = consumer_thread.join();
}
