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
use std::cell::Cell;

use core::line::{Line, LineCollection, Parser as LineParser};
use core::filter::Filter;

static DEFAULT_REVERSE_INDEX: usize = 0;
static MAX_LINES_RENDERED: usize = 2_000;

pub struct Buffer {
    pub filter: Filter,
    pub reverse_index: Cell<usize>,
}

impl Buffer {
    pub fn new(filter: Filter) -> Buffer {
        Buffer {
            filter: filter,
            reverse_index: Cell::new(DEFAULT_REVERSE_INDEX),
        }
    }

    pub fn with_lines<'a>(&'a self, lines: &'a LineCollection) -> BufferLines<'a> {
        BufferLines::new(self, lines)
    }

    pub fn increment_reverse_index(&self, value: i32, max_value: usize) {
        self.set_reverse_index(self.reverse_index.get() as i32 + value, max_value);
    }

    pub fn set_reverse_index(&self, value: i32, max_value: usize) {
        self.reverse_index.set(min(max(0, value) as usize, max_value));
    }

    pub fn is_scrolled(&self) -> bool {
        self.reverse_index.get() != DEFAULT_REVERSE_INDEX
    }

    pub fn reset_reverse_index(&self) {
        self.reverse_index.set(DEFAULT_REVERSE_INDEX);
    }
}

pub struct BufferLines<'a> {
    lines: &'a LineCollection,
    pub buffer: &'a Buffer,
    pub width: usize,
}

impl<'a> BufferLines<'a> {
    fn new(buffer: &'a Buffer, lines: &'a LineCollection) -> BufferLines<'a> {
        BufferLines {
            buffer: buffer,
            lines: lines,
            width: 0,
        }
    }
}

impl<'a> IntoIterator for &'a BufferLines<'a> {
    type Item = &'a Line;
    type IntoIter = ::std::vec::IntoIter<&'a Line>;

    fn into_iter(self) -> Self::IntoIter {
        let mut estimated_height = 0;

        let height_within_boundary = |line: &&Line| -> bool {
            estimated_height += line.guess_height(self.width);
            estimated_height <= MAX_LINES_RENDERED
        };

        let mut lines = self.lines
            .entries
            .iter()
            .parse(self.buffer.filter.clone())
            .take_while(height_within_boundary)
            .collect::<Vec<_>>();
        lines.reverse();
        lines.into_iter()
    }
}

pub struct BufferCollection {
    items: Vec<Buffer>,
    index: usize,
}

impl BufferCollection {
    pub fn from_filters(filters: Vec<Filter>) -> BufferCollection {
        let items = filters.iter().map(|e| Buffer::new(e.clone())).collect();

        BufferCollection {
            items: items,
            index: 0,
        }
    }

    pub fn selected_item(&self) -> &Buffer {
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
