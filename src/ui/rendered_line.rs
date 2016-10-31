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

use std::ops::Index;

pub struct RenderedLine {
    pub height: i32,
    pub found_matches: Option<Vec<usize>>,
}

impl RenderedLine {
    fn new(height: i32, found_matches: Option<Vec<usize>>) -> RenderedLine {
        RenderedLine {
            height: height,
            found_matches: found_matches,
        }
    }

    pub fn match_count(&self) -> usize {
        self.found_matches.as_ref().unwrap().len()
    }
}

pub struct RenderedLineCollection {
    pub entries: Vec<RenderedLine>,
}

impl RenderedLineCollection {
    pub fn default() -> RenderedLineCollection {
        RenderedLineCollection { entries: vec![] }
    }

    pub fn create(&mut self, height: i32, found_matches: Option<Vec<usize>>) {
        let entry = RenderedLine::new(height, found_matches);
        self.entries.push(entry);
    }

    pub fn height(&self) -> i32 {
        self.entries.iter().height()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn buffer_reverse_index(&self, line_index: usize, match_index: usize) -> i32 {
        let offset = self.entries[line_index].found_matches.as_ref().unwrap()[match_index];
        self.entries.iter().skip(line_index).height() - offset as i32
    }

    pub fn height_up_to_index(&self, index: usize) -> i32 {
        self.entries.iter().take(index).height()
    }
}

trait Height {
    fn height(self) -> i32;
}

impl<'a, I> Height for I
    where I: Iterator<Item = &'a RenderedLine>
{
    fn height(self) -> i32 {
        self.fold(0, |sum, current| sum + current.height)
    }
}

pub struct MatchedLine {
    pub line: usize,
    pub matches: usize,
}

pub trait FindMatch {
    fn find_match(mut self) -> Option<MatchedLine>;
}

impl<'a, I> FindMatch for I
    where I: Iterator<Item = (usize, &'a RenderedLine)>
{
    fn find_match(mut self) -> Option<MatchedLine> {
        if let Some(value) = self.find(|&index_and_line| index_and_line.1.found_matches.is_some()) {
            Some(MatchedLine {
                line: value.0,
                matches: value.1.match_count() - 1,
            })
        } else {
            None
        }
    }
}

impl Index<usize> for RenderedLineCollection {
    type Output = RenderedLine;

    fn index<'a>(&'a self, _index: usize) -> &'a RenderedLine {
        &self.entries[_index]
    }
}
