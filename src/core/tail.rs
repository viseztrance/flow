/**
 * Flow - Realtime log analyzer
 * Copyright (C) 2016 Daniel Mircea
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use std::io::prelude::{Read, Seek};
use std::fs::File;
use std::io::SeekFrom;
use std::process;
use std::sync::atomic::Ordering;
use std::time::Duration;
use std::thread::sleep;

use core::runner::RUNNING;

pub struct Tail {
    file: File,
    start_of_file_reached: bool,
}

impl Tail {
    pub fn new(file_path: String) -> Tail {
        let file_handle = match File::open(&file_path) {
            Ok(value) => value,
            Err(message) => {
                let message = format!("`{}` couldn't be opened - {}", file_path, message);
                critical_quit!(message);
            }
        };

        Tail {
            file: file_handle,
            start_of_file_reached: false,
        }
    }

    pub fn watch<F>(&mut self, callback: F)
        where F: Fn(Vec<String>)
    {
        while running!() {
            callback(self.read_to_end());
            sleep(Duration::from_millis(50));
        }
    }

    pub fn read_lines(&mut self, lines: usize) -> Vec<String> {
        let estimated_required_bytes = lines * 512;
        self.read_lines_conditionally(estimated_required_bytes, lines)
    }

    pub fn read_to_end(&mut self) -> Vec<String> {
        let mut buffer = String::new();
        let _ = self.file.read_to_string(&mut buffer);
        buffer.lines().map(|x| x.to_string()).collect()
    }

    fn read_bytes_from_file_end(&mut self, bytes: usize) -> Vec<String> {
        let file_size = self.read_file_size();
        let mut seekable_bytes = bytes;
        if bytes > file_size {
            self.start_of_file_reached = true;
            seekable_bytes = file_size;
        }
        let _ = self.file.seek(SeekFrom::End(-(seekable_bytes as i64)));
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
            }
            None => self.read_lines_conditionally(bytes * 2, target_lines),
        }
    }

    fn excess_lines_at_beggining_of_buffer(&self,
                                           buffer: &[String],
                                           target_lines: usize)
                                           -> Option<usize> {
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
