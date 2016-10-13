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

use ncurses::*;

pub struct Plane {
    pub lines: Vec<i32>,
    pub height: i32,
    pub width: i32
}

impl Plane {
    pub fn new() -> Plane {
        Plane {
            lines: Vec::new(),
            height: LINES,
            width: COLS
        }
    }

    pub fn resize(&mut self) {
        getmaxyx(stdscr, &mut self.height, &mut self.width);
    }

    pub fn virtual_height(&self) -> i32 {
        self.lines.iter().sum()
    }

    pub fn viewport(&self, scroll_offset: usize) -> (usize, usize) {
        let limit = scroll_offset + self.height as usize - 1;
        let mut start = None;
        let mut end = None;
        let mut offset = 0;

        for (i, line_height) in self.lines.iter().rev().enumerate() {
            if start.is_none() && offset >= scroll_offset {
                start = Some(i);
            }

            offset += *line_height as usize;

            if end.is_none() && offset >= limit {
                end = Some(i);
                break;
            }
        }
        if end.is_none() {
            end = Some(0);
        }

        let count = self.lines.len() - 1;
        (count - end.unwrap(), count - start.unwrap())
    }
}
