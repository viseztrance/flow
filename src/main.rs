extern crate notify;
extern crate ncurses;

use std::env;
use std::process;

use std::sync::{Arc, Mutex};
use std::thread;

use flow::tail::Tail;
use flow::flow::Flow;

pub mod flow {
    pub mod flow;
    pub mod tail;
    pub mod buffer;
    pub mod ui;
}

fn main() {
    let args: Vec<_> = env::args().collect();

    let target = match args.get(1) {
        Some(value) => value.to_string(),
        None => {
            println!("No file specified");
            process::exit(0)
        }
    };

    let line_count = match args.get(2) {
        Some(value) => value.parse::<usize>().unwrap(),
        None => 4
    };

    run(target, line_count);
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
