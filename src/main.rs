extern crate notify;
extern crate ncurses;
extern crate getopts;

use std::env;

use std::sync::{Arc, Mutex};
use std::thread;

use settings_builder::SettingsBuilder;
use flow::tail::Tail;
use flow::flow::Flow;

pub mod settings_builder;
pub mod flow {
    pub mod flow;
    pub mod tail;
    pub mod buffer;
    pub mod ui;
}

fn main() {
    let args = env::args().collect();

    let parsed_args = SettingsBuilder::new(args).make();
    run(parsed_args.path_to_target_file, parsed_args.line_count);
}

fn run(target: String, line_count: usize) {
    let mut tail = Tail::new(target);

    let lines = Arc::new(Mutex::new(tail.read_lines(line_count)));

    let reader_lines = lines.clone();
    let reader_thread = thread::spawn(move || {
        tail.watch(|data| reader_lines.lock().unwrap().extend(data));
    });

    let consumer_lines = lines.clone();
    let consumer_thread = thread::spawn(move || {
        let mut flow = Flow::new(vec!["lorem", "ipsum!"]);
        flow.render();
        flow.process(consumer_lines);
        flow.terminate();
    });

    let _ = reader_thread.join();
    let _ = consumer_thread.join();
}
