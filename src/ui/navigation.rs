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

use ui::menu::Menu;
use ui::search::Search;

pub static HEIGHT: i32 = 1;

#[derive(PartialEq)]
pub enum State {
    Menu,
    Search,
}

pub struct Navigation {
    pub menu: Menu,
    pub search: Search,
    pub state: State,
}

impl Navigation {
    pub fn new(position_x: i32, position_y: i32, menu_item_names: &[String]) -> Navigation {
        Navigation {
            menu: Menu::new(position_x, position_y, menu_item_names),
            search: Search::new(position_x, position_y),
            state: State::Menu,
        }
    }

    pub fn render(&self) {
        self.search.render();
        self.menu.render();
        self.handle_visibility();
    }

    pub fn destroy(&self) {
        self.menu.destroy();
    }

    pub fn change_state(&mut self, new_state: State) -> bool {
        if self.state == new_state {
            false
        } else {
            self.state = new_state;
            self.handle_visibility();
            true
        }
    }

    pub fn resize(&self, position_x: i32, position_y: i32) {
        mvwin(self.search.window, position_y, position_x);
        mvwin(self.search.input_field.window,
              position_y,
              position_x + HEIGHT);
        wrefresh(self.search.input_field.window);
        mvwin(self.menu.window, position_y, position_x);

        self.render();
    }

    fn handle_visibility(&self) {
        match self.state {
            State::Menu => {
                self.search.hide();
                self.menu.show();
            }
            State::Search => {
                self.menu.hide();
                self.search.show();
            }
        }

        update_panels();
        doupdate();
    }
}
