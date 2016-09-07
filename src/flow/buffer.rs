use std::collections::VecDeque;
use std::cell::RefCell;

use regex::Regex;

pub struct Buffer {
    pub filter: BufferFilter,
    pub reverse_index: usize
}

impl Buffer {
    pub fn new(filter: BufferFilter) -> Buffer {
        Buffer {
            filter: filter,
            reverse_index: 0,
        }
    }

    pub fn parse<'a>(&self, lines: &'a VecDeque<String>) -> (Box<Iterator<Item=&'a String> + 'a>, usize) {
        let regex = Regex::new(&self.filter.rule.clone().unwrap_or(".*".to_string())).unwrap();

        let parsed_lines = lines.iter().filter(move |line| regex.is_match(line));
        (Box::new(parsed_lines), self.reverse_index)
    }

    pub fn increment_reverse_index(&mut self, value: usize, lines: &VecDeque<String>) {
        if self.reverse_index + value < lines.len() {
            self.reverse_index += value;
        } else {
            self.reverse_index = lines.len();
        }
    }

    pub fn decrement_reverse_index(&mut self, value: usize) {
        let future_index = self.reverse_index as isize - value as isize;

        if future_index > 0 {
            self.reverse_index -= value;
        } else {
            self.reverse_index = 0;
        }
    }
}

#[derive(RustcDecodable, Clone)]
pub struct BufferFilter {
    pub name: String,
    pub rule: Option<String>,
}

pub struct BufferCollection {
    items: Vec<RefCell<Buffer>>,
    index: usize,
}

impl BufferCollection {
    pub fn from_filters(filters: Vec<BufferFilter>) -> BufferCollection {
        let items = filters.iter().map(|e| RefCell::new(Buffer::new(e.clone()))).collect();

        BufferCollection {
            items: items,
            index: 0
        }
    }

    pub fn selected_item(&self) -> &RefCell<Buffer> {
        self.items.get(self.index).unwrap()
    }

    pub fn select_left_item(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        }
    }

    pub fn select_right_item(&mut self) {
        if self.index + 1 < self.items.len() {
            self.index += 1;
        }
    }
}
