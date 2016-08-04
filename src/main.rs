extern crate notify;

use std::env;
use std::process;

use flow::flow::Flow;

pub mod flow {
    pub mod flow;
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

    let mut flow = Flow::new(target);
    flow.read(lines);
    flow.watch()
}
