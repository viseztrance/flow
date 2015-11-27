extern crate notify;

use notify::{RecommendedWatcher, Error, Watcher};
use std::sync::mpsc::channel;

use std::env;
use std::process;

use flow::tail::Tail;
pub mod flow {
    pub mod tail;
}

fn main() {
    let args: Vec<_> = env::args().collect();

    let target = match args.get(1) {
        Some(value) => value,
        None => {
            println!("No file specified");
            process::exit(0)
        }
    };

    let lines = match args.get(2) {
        Some(value) => value.parse::<usize>().unwrap(),
        None => 4
    };

    let mut tail = Tail::new(target);
    let data = tail.read_lines(lines);
    print!("{}", data);

    let (tx, rx) = channel();
    let mut w: Result<RecommendedWatcher, Error> = Watcher::new(tx);

    loop {
        match w {
            Ok(ref mut watcher) => {
                watcher.watch(target);
                match rx.recv() {
                    _ => print!("{}", tail.read_to_end())
                }
            },
            Err(ref e) => panic!("Error while scanning for changes: {message:?}", message = e)
        }
    }
}
