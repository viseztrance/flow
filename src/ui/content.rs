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

use std::cell::RefCell;

use ncurses::*;

use ui::color::COLOR_DEFAULT;

static WINDOW_HEIGHT: i32 = 2500;

pub struct Content {
    pub window: WINDOW,
    pub state: RefCell<State>,
}

impl Content {
    pub fn new(width: i32) -> Content {
        Content {
            window: newpad(WINDOW_HEIGHT, width),
            state: RefCell::new(State::default()),
        }
    }

    pub fn clear(&self) {
        wclear(self.window);
    }

    pub fn resize(&self, width: i32) {
        wresize(self.window, WINDOW_HEIGHT, width);
        wrefresh(self.window);
    }

    pub fn height(&self) -> i32 {
        let mut current_x: i32 = 0;
        let mut current_y: i32 = 0;
        getyx(self.window, &mut current_y, &mut current_x);

        current_y
    }

    pub fn calculate_height_change<F>(&self, callback: F) -> i32
        where F: Fn()
    {
        let initial_height = self.height();
        callback();
        self.height() - initial_height
    }
}

pub struct State {
    pub attributes: Vec<(usize, fn() -> u64)>,
    pub foreground: i16,
    pub background: i16,
    pub highlighted_line: usize,
    pub highlighted_match: usize,
}

impl State {
    pub fn default() -> State {
        State {
            attributes: vec![],
            foreground: COLOR_DEFAULT,
            background: COLOR_DEFAULT,
            highlighted_line: 0,
            highlighted_match: 0,
        }
    }

    pub fn remove_attribute(&mut self, attr_id: usize) {
        let index_option = self.attributes.iter().position(|cur| cur.0 == attr_id);

        if let Some(index) = index_option {
            self.attributes.remove(index);
        }
    }
}
