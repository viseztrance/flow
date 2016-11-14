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

use ui::readline;

static OPTIONS_WIDTH: i32 = 29;
static WITH_MATCHES_COLOR_PAIR_ID: i16 = 1;
static NO_MATCHES_COLOR_PAIR_ID: i16 = 4;

#[derive(PartialEq)]
pub enum Highlight {
    VisibleOrLast,
    Next,
    Previous,
    Current,
}

pub struct Query {
    pub text: String,
    pub filter_mode: bool,
    pub highlight: Highlight,
}

pub struct Search {
    pub window: WINDOW,
    pub options: Options,
    pub input_field: InputField,
    pub matches_found: bool,
    panel: PANEL,
}

impl Search {
    pub fn new(position_x: i32, position_y: i32) -> Search {
        let window = newwin(0, 0, position_x, position_y);

        Search {
            window: window,
            options: Options::new(window),
            input_field: InputField::new(window),
            panel: new_panel(window),
            matches_found: false,
        }
    }

    pub fn render(&self) {
        let color_pair = COLOR_PAIR(self.color_pair_id());

        wbkgd(self.window, color_pair);
        self.input_field.render(color_pair);
        self.options.render(color_pair);
        wrefresh(self.window);
        readline::move_cursor();
    }

    pub fn resize(&self, container_width: i32, offset: i32) {
        mvwin(self.window, offset, 0);

        self.input_field.resize(container_width, offset);
        self.options.resize(container_width);
    }

    pub fn build_query(&self, highlight: Highlight) -> Option<Query> {
        if self.input_field.is_empty() {
            None
        } else {
            Some(Query {
                text: self.input_field.text.borrow().clone(),
                filter_mode: self.options.filter_mode,
                highlight: highlight,
            })
        }
    }

    pub fn toggle_filter_mode(&mut self) {
        self.options.filter_mode = !self.options.filter_mode;
        self.render();
    }

    pub fn show(&self) {
        self.render();
        curs_set(CURSOR_VISIBILITY::CURSOR_VERY_VISIBLE);
        show_panel(self.panel);
    }

    pub fn hide(&self) {
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        hide_panel(self.panel);
    }

    fn color_pair_id(&self) -> i16 {
        if !self.matches_found && self.input_field.text.borrow().len() > 0 {
            NO_MATCHES_COLOR_PAIR_ID
        } else {
            WITH_MATCHES_COLOR_PAIR_ID
        }
    }
}
#[derive(PartialEq)]
pub enum State {
    Changed,
    Unchanged,
}

pub struct InputField {
    pub window: WINDOW,
    text: RefCell<String>,
}

impl InputField {
    fn new(parent_window: WINDOW) -> InputField {
        let window = derwin(parent_window, 1, COLS() - OPTIONS_WIDTH, 0, 1);
        syncok(window, true);

        InputField {
            window: window,
            text: RefCell::new(String::new()),
        }
    }

    fn render(&self, color_pair: u32) {
        wclear(self.window);
        wbkgd(self.window, color_pair);
        wattron(self.window, color_pair);
    }

    fn resize(&self, container_width: i32, offset: i32) {
        wresize(self.window, 1, container_width - OPTIONS_WIDTH);
        mvwin(self.window, offset, 1);
        wrefresh(self.window);
    }

    pub fn read(&self, keys: Vec<i32>) -> State {
        for key in keys {
            readline::forward_input(key);
        }

        let pending_text = readline::read_buffer().to_string();
        let mut current_text = self.text.borrow_mut();

        if *current_text == pending_text {
            State::Unchanged
        } else {
            *current_text = pending_text;
            State::Changed
        }
    }

    fn is_empty(&self) -> bool {
        self.text.borrow().is_empty()
    }
}

pub struct Options {
    pub window: WINDOW,
    filter_mode: bool,
}

impl Options {
    fn new(parent_window: WINDOW) -> Options {
        Options {
            window: derwin(parent_window, 1, OPTIONS_WIDTH, 0, COLS() - OPTIONS_WIDTH),
            filter_mode: false,
        }
    }

    fn render(&self, color_pair: u32) {
        wclear(self.window);
        readline::handle_redisplay();
        wbkgd(self.window, color_pair);

        self.print_navigation();
        self.print_filter(color_pair);
    }

    fn resize(&self, container_width: i32) {
        wresize(self.window, 1, OPTIONS_WIDTH);
        mvderwin(self.window, 0, container_width - OPTIONS_WIDTH);
        wrefresh(self.window);
    }

    fn print_navigation(&self) {
        wprintw(self.window, " / ");
        self.make_shortcut('N');
        wprintw(self.window, "ext / ");
        self.make_shortcut('P');
        wprintw(self.window, "rev");
    }

    fn print_filter(&self, color_pair: u32) {
        wprintw(self.window, " / ");
        if self.filter_mode {
            wattron(self.window, COLOR_PAIR(2));
        }
        wprintw(self.window, "Filter ");
        self.make_shortcut('M');
        wprintw(self.window, "ode");
        if self.filter_mode {
            wattroff(self.window, color_pair);
        }
    }

    fn make_shortcut(&self, letter: char) {
        wattron(self.window, A_UNDERLINE());
        waddch(self.window, letter as u32);
        wattroff(self.window, A_UNDERLINE());
    }
}
