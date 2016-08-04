use notify::{RecommendedWatcher, Error, Watcher};
use std::sync::mpsc::channel;

use flow::tail::Tail;

pub struct Flow<'a> {
    tail: Tail,
    target: &'a str
}

impl<'a> Flow<'a> {
    pub fn new<'b>(target: &str) -> Flow {
        Flow {
            tail: Tail::new(target),
            target: target
        }
    }

    pub fn read(&mut self, line_count: usize) {
        let data = self.tail.read_lines(line_count);
        print!("{}", data);
    }

    pub fn watch(&mut self) {
        let (tx, rx) = channel();
        let mut w: Result<RecommendedWatcher, Error> = Watcher::new(tx);

        loop {
            match w {
                Ok(ref mut watcher) => {
                    watcher.watch(self.target);
                    match rx.recv() {
                        _ => print!("{}", self.tail.read_to_end())
                    }
                },
                Err(ref e) => panic!("Error while scanning for changes: {message:?}", message = e)
            }
        }
    }
}
