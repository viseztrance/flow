use std::io::prelude::*;
use std::fs::File;
use std::io::SeekFrom;

pub struct Tail {
    file: File,
    start_of_file_reached: bool
}

impl Tail {
    pub fn new(file_path: &str) -> Tail {
        Tail {
            file: File::open(file_path).unwrap(),
            start_of_file_reached: false
        }
    }

    pub fn read_lines(&mut self, lines: usize) -> String {
        let estimated_required_bytes = lines * 512;
        self.read_lines_conditionally(estimated_required_bytes, lines)
    }

    pub fn read_to_end(&mut self) -> String {
        let mut buffer = String::new();
        &self.file.read_to_string(&mut buffer);
        buffer
    }

    fn read_bytes_from_file_end(&mut self, bytes: usize) -> String {
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

    fn read_lines_conditionally(&mut self, bytes: usize, target_lines: usize) -> String {
        let buffer = self.read_bytes_from_file_end(bytes);

        match self.excess_lines_at_beggining_of_buffer(&buffer, target_lines) {
            Some(count) => {
                let mut data = String::new();
                for line in buffer.lines().skip(count) {
                    data.push_str(line);
                    data.push_str("\n");
                }
                data
            },
            None => self.read_lines_conditionally(bytes * 2, target_lines)
        }
    }

    fn excess_lines_at_beggining_of_buffer(&self, buffer: &String, target_lines: usize) -> Option<usize> {
        let count = buffer.lines().count();
        if count >= target_lines {
            Some(count - target_lines)
        } else if self.start_of_file_reached {
            Some(0)
        } else {
            None
        }
    }
}
