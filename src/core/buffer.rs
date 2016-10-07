use std::cmp::{min, max};
use std::cell::RefCell;

use core::line::{Line, LineCollection};
use core::filter::Filter;

static DEFAULT_REVERSE_INDEX: usize = 0;

#[derive(PartialEq)]
pub enum ScrollState {
    Unchanged,
    Changed
}

pub struct Buffer {
    pub filter: Filter,
    pub reverse_index: usize
}

impl Buffer {
    pub fn new(filter: Filter) -> Buffer {
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

pub struct BufferCollection {
    items: Vec<RefCell<Buffer>>,
    index: usize,
}

impl BufferCollection {
    pub fn from_filters(filters: Vec<Filter>) -> BufferCollection {
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
