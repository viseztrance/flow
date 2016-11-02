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

use std::cmp::max;
use std::collections::VecDeque;

use unicode_width::UnicodeWidthStr;

use utils::ansi_decoder::{ComponentCollection, AnsiStr};

pub struct Line {
    pub content_without_ansi: String,
    pub components: Option<ComponentCollection>,
    pub width: usize,
}

impl Line {
    fn new(content: String) -> Line {
        let has_ansi = content.has_ansi_escape_sequence();

        let (content_without_ansi, components) = if has_ansi {
            (content.strip_ansi(), Some(content.to_components()))
        } else {
            (content, None)
        };

        Line {
            width: content_without_ansi.width(),
            content_without_ansi: content_without_ansi,
            components: components,
        }
    }

    pub fn guess_height(&self, container_width: usize) -> usize {
        max(1, self.width / container_width)
    }

    pub fn matches_for(&self, text: &str) -> Vec<(usize, &str)> {
        self.content_without_ansi.match_indices(text).collect()
    }

    pub fn contains(&self, text: &str) -> bool {
        self.content_without_ansi.contains(text)
    }
}

pub struct LineCollection {
    pub entries: VecDeque<Line>,
    capacity: usize,
}

impl LineCollection {
    pub fn new(capacity: usize) -> LineCollection {
        LineCollection {
            entries: VecDeque::new(),
            capacity: capacity,
        }
    }

    fn clear_excess(&mut self) {
        while self.entries.len() > self.capacity {
            self.entries.pop_front();
        }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    fn add(&mut self, item: String) {
        self.entries.push_back(Line::new(item));
    }
}

impl Extend<String> for LineCollection {
    fn extend<T: IntoIterator<Item = String>>(&mut self, iter: T) {
        for item in iter {
            self.add(item);
        }

        self.clear_excess();
    }
}
