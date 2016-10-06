use std::cmp::{min, max};

use regex::Regex;
use rustc_serialize::{Decodable, Decoder};

use core::line::{Line, LineCollection};
use std::cell::RefCell;

static DEFAULT_REVERSE_INDEX: usize = 0;

#[derive(PartialEq)]
pub enum ScrollState {
    Unchanged,
    Changed
}

pub struct Buffer {
    pub filter: BufferFilter,
    pub reverse_index: usize
}

impl Buffer {
    pub fn new(filter: BufferFilter) -> Buffer {
        Buffer {
            filter: filter,
            reverse_index: DEFAULT_REVERSE_INDEX
        }
    }

    pub fn parse<'a>(&self, lines: &'a LineCollection) -> (Box<Iterator<Item=&'a Line> + 'a>, usize) {
        let mut filter = self.filter.clone();

        let parsed_lines = lines.entries.iter().filter(move |line| {
            filter.is_match(&line.content_without_ansi)
        });
        (Box::new(parsed_lines), self.reverse_index)
    }

    pub fn adjust_reverse_index(&mut self, value: i32, lines: &LineCollection) -> ScrollState {
        if value == DEFAULT_REVERSE_INDEX as i32 {
            return ScrollState::Unchanged;
        }

        let new_reverse_index = self.reverse_index as i64 + value as i64;
        self.reverse_index = min(max(0, new_reverse_index), lines.len() as i64) as usize;
        ScrollState::Changed
    }

    pub fn is_scrolled(&self) -> bool {
        self.reverse_index != DEFAULT_REVERSE_INDEX
    }

    pub fn reset_reverse_index(&mut self) {
        self.reverse_index = DEFAULT_REVERSE_INDEX;
    }
}

#[derive(Clone)]
pub struct BufferFilter {
    pub name: String,
    pub starts_with: Option<Regex>,
    pub contains: Regex,
    pub ends_with: Option<Regex>
}

impl BufferFilter {
    pub fn is_match(&mut self, text: &String) -> bool {
        // TODO: add state
        self.contains.is_match(&text)
    }
}

impl Decodable for BufferFilter {
    fn decode<D: Decoder>(d: &mut D) -> Result<BufferFilter, D::Error> {
        d.read_struct("BufferFilter", 2, |d| {
            let name = try!(d.read_struct_field("name", 0, |d| d.read_str()));

            let starts_with = match d.read_struct_field("starts_with", 1, |d| d.read_str()) {
                Ok(val) => Some(Regex::new(&val).unwrap()),
                Err(_) => None
            };

            let contains = match d.read_struct_field("contains", 2, |d| d.read_str()) {
                Ok(val) => Regex::new(&val),
                Err(_) => Regex::new(".*")
            }.unwrap();

            let ends_with = match d.read_struct_field("ends_with", 3, |d| d.read_str()) {
                Ok(val) => Some(Regex::new(&val).unwrap()),
                Err(_) => None
            };

            Ok(BufferFilter {
                name: name,
                starts_with: starts_with,
                contains: contains,
                ends_with: ends_with
            })
        })
    }
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

    pub fn select_previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        }
    }

    pub fn select_next(&mut self) {
        if self.index + 1 < self.items.len() {
            self.index += 1;
        }
    }
}
