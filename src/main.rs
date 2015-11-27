use std::env;
use std::process;
use std::time::Duration;
use std::thread;

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
    loop {
        thread::sleep(Duration::from_millis(1000));
        print!("{}", tail.read_to_end());
    }
}
