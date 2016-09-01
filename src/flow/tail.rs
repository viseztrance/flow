use std::io::prelude::*;
use std::fs::File;
use std::io::SeekFrom;
use std::process;

use notify::{RecommendedWatcher, Error, Watcher};
use std::sync::mpsc::channel;

pub struct Tail {
    file: File,
    path: String,
    start_of_file_reached: bool
}

impl Tail {
    pub fn new(file_path: String) -> Tail {
        let file_handler = match File::open(&file_path) {
            Ok(value) => value,
            Err(message) => {
                println!("`{}` couldn't be opened - {}", file_path, message);
                process::exit(2);
            }
        };

        Tail {
            file: file_handler,
            path: file_path,
            start_of_file_reached: false
        }
    }

    pub fn watch<F>(&mut self, callback: F) where F : Fn(Vec<String>) {
        let (tx, rx) = channel();
        let mut w: Result<RecommendedWatcher, Error> = Watcher::new(tx);

        loop {
            match w {
                Ok(ref mut watcher) => {
                    let _ = watcher.watch(&self.path);

                    match rx.recv() {
                        _ => {
                            let data = self.read_to_end();
                            callback(data);
                        }
                    }
                },
                Err(ref e) => panic!("Error while scanning for changes: {message:?}", message = e)
            }
        }
    }

    pub fn read_lines(&mut self, lines: usize) -> Vec<String> {
        let estimated_required_bytes = lines * 512;
        self.read_lines_conditionally(estimated_required_bytes, lines)
    }

    pub fn read_to_end(&mut self) -> Vec<String> {
        let mut buffer = String::new();
        &self.file.read_to_string(&mut buffer);
        buffer.lines().map(|x| x.to_string()).collect()
    }

    fn read_bytes_from_file_end(&mut self, bytes: usize) -> Vec<String> {
        let file_size = self.read_file_size();
        let mut seekable_bytes = bytes;
        if bytes > file_size {
            self.start_of_file_reached = true;
            seekable_bytes = file_size;
        }
        &self.file.seek(SeekFrom::End(-(seekable_bytes as i64)));
        self.read_to_end()
    }

    fn read_file_size(&self) -> usize {
        *(&self.file.metadata().unwrap().len()) as usize
    }

    fn read_lines_conditionally(&mut self, bytes: usize, target_lines: usize) -> Vec<String> {
        let buffer = self.read_bytes_from_file_end(bytes);

        match self.excess_lines_at_beggining_of_buffer(&buffer, target_lines) {
            Some(count) => {
                let (_, result) = buffer.split_at(count);
                result.to_vec()
            },
            None => self.read_lines_conditionally(bytes * 2, target_lines)
        }
    }

    fn excess_lines_at_beggining_of_buffer(&self, buffer: &Vec<String>, target_lines: usize) -> Option<usize> {
        let count = buffer.len();
        if count >= target_lines {
            Some(count - target_lines)
        } else if self.start_of_file_reached {
            Some(0)
        } else {
            None
        }
    }
}
