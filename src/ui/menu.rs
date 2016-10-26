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

pub struct Menu {
    pub window: WINDOW,
    panel: PANEL,
    object: MENU,
    items: Vec<ITEM>
}

impl Menu {
    pub fn new(position_x: i32, position_y: i32, item_names: &Vec<String>) -> Menu {
        let window = newwin(0, 0, position_x, position_y);

        let mut items = vec![];

        for name in item_names {
            items.push(new_item(format!(" {} ", name), String::new()));
        }

        Menu {
            window: window,
            panel: new_panel(window),
            object: new_menu(&mut items),
            items: items
        }
    }

    pub fn select(&self, item: i32) {
        menu_driver(self.object, item);
        pos_menu_cursor(self.object);
        wrefresh(self.window);
    }

    pub fn render(&self) {
        set_menu_win(self.object, self.window);
        set_menu_sub(self.object, derwin(self.window, 0, 0, 0, 0));

        menu_opts_off(self.object, O_SHOWDESC);
        set_menu_mark(self.object, "");
        set_menu_fore(self.object, COLOR_PAIR(1));
        set_menu_back(self.object, COLOR_PAIR(2));
        set_menu_format(self.object, 1, self.items.len() as i32);
        post_menu(self.object);

        refresh();
        wbkgd(self.window, COLOR_PAIR(2));
        wrefresh(self.window);
    }

    pub fn show(&self) {
        show_panel(self.panel);
    }

    pub fn hide(&self) {
        hide_panel(self.panel);
    }

    pub fn destroy(&self) {
        unpost_menu(self.object);

        for item in &self.items {
            free_item(*item);
        }

        free_menu(self.object);
    }
}
