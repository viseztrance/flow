use notify::{RecommendedWatcher, Error, Watcher};
use std::sync::mpsc::channel;

use flow::tail::Tail;
use flow::buffer::Buffer;

pub struct Flow {
    tail: Tail,
    target: String,
    lines: Vec<String>,
    buffers: Vec<Buffer>
}

impl Flow {
    pub fn new(target: String) -> Flow {
        Flow {
            tail: Tail::new(target.clone()),
            target: target,
            lines: vec![],
            buffers: vec![Buffer::new("my first buffer".to_string()), Buffer::new("my last buffer".to_string())]
        }
    }

    pub fn read(&mut self, line_count: usize) {
        let data = self.tail.read_lines(line_count);
        self.process(data);
    }

    pub fn watch(&mut self) {
        let (tx, rx) = channel();
        let mut w: Result<RecommendedWatcher, Error> = Watcher::new(tx);

        loop {
            match w {
                Ok(ref mut watcher) => {
                    watcher.watch(&self.target);

                    match rx.recv() {
                        _ => {
                            let data = self.tail.read_to_end();
                            self.process(data);
                        }
                    }
                },
                Err(ref e) => panic!("Error while scanning for changes: {message:?}", message = e)
            }
        }
    }

    pub fn process(&mut self, data: Vec<String>) {
        // println!("{:?}", data);
        self.lines.extend(data);
        self.buffers[0].render(&self.lines);
    }
}
