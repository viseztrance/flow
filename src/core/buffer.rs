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

use std::cmp::{min, max};
use std::cell::RefCell;

use core::line::{Line, LineCollection};
use core::filter::Filter;

static DEFAULT_REVERSE_INDEX: usize = 0;

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

    pub fn parse<'a>(&self, lines: &'a LineCollection) -> (Box<DoubleEndedIterator<Item=&'a Line> + 'a>, usize) {
        let mut filter = self.filter.clone();

        let parsed_lines = lines.entries.iter().filter(move |line| {
            filter.is_match(&line.content_without_ansi)
        });
        (Box::new(parsed_lines), self.reverse_index)
    }

    pub fn adjust_reverse_index(&mut self, value: i32, max_value: i32) {
        let new_reverse_index = self.reverse_index as i32 + value;
        self.reverse_index = min(max(0, new_reverse_index), max_value) as usize;
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
