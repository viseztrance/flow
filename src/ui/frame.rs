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

use core::buffer::BufferLines;
use ui::readline;
use ui::color;
use ui::input::read_key;
use ui::event::{EventBuilder, Event};
use ui::navigation::{Navigation, HEIGHT as NAVIGATION_HEIGHT};
use ui::content::Content;
use ui::printer::LinesPrinter;
use ui::search::Query;
use ui::rendered_line::RenderedLineCollection;

pub static NORMAL_HIGHLIGHT_COLOR: i16 = 5;
pub static CURRENT_HIGHLIGHT_COLOR: i16 = 6;

pub struct Frame {
    pub width: i32,
    pub height: i32,
    pub rendered_lines: RenderedLineCollection,
    pub navigation: Navigation,
    pub content: Content,
}

impl Frame {
    pub fn new(menu_item_names: Vec<String>) -> Frame {
        // Init order is important
        env_init();
        readline::init();
        ncurses_init();
        color::generate_pairs();

        Frame {
            width: COLS(),
            height: LINES(),
            rendered_lines: RenderedLineCollection::default(),
            navigation: Navigation::new(LINES() - NAVIGATION_HEIGHT, 0, &menu_item_names),
            content: Content::new(COLS()),
        }
    }

    pub fn render(&self) {
        readline::render("Search:", self.navigation.search.input_field.window);

        self.navigation.render();
    }

    pub fn select_left_menu_item(&self) {
        self.navigation.menu.select(REQ_LEFT_ITEM);
    }

    pub fn select_right_menu_item(&self) {
        self.navigation.menu.select(REQ_RIGHT_ITEM);
    }

    pub fn destroy(&self) {
        self.navigation.destroy();
        endwin();
        readline::terminate();
    }

    pub fn resize(&mut self) {
        getmaxyx(stdscr(), &mut self.height, &mut self.width);

        self.content.resize(self.width);
        self.navigation.resize(self.width, self.content_height());
    }

    pub fn print(&mut self, buffer_lines: &mut BufferLines, query: Option<Query>) {
        buffer_lines.set_context(self.width as usize, query);

        LinesPrinter::new(self, buffer_lines).draw();
        self.scroll(buffer_lines.buffer.reverse_index.get() as i32);
    }

    pub fn scroll(&self, reversed_offset: i32) {
        let offset = self.rendered_lines.height() - self.height + NAVIGATION_HEIGHT -
                     reversed_offset;
        prefresh(self.content.window,
                 offset,
                 0,
                 0,
                 0,
                 self.content_height() - 1,
                 self.width);
    }

    pub fn watch(&self) -> Event {
        let (input, key) = read_key();
        EventBuilder::new(input, key).construct(&self.navigation.state)
    }

    pub fn reset(&mut self) {
        self.rendered_lines.clear();
        self.content.clear();
        self.navigation.search.matches_found = false;
    }

    pub fn max_scroll_value(&self) -> usize {
        (self.rendered_lines.height() - self.height) as usize
    }

    pub fn content_height(&self) -> i32 {
        self.height - NAVIGATION_HEIGHT
    }
}

fn env_init() {
    ::std::env::set_var("ESCDELAY", "25");
    setlocale(LcCategory::all, "");
}

fn ncurses_init() {
    initscr();
    start_color();
    use_default_colors();
    cbreak();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    halfdelay(1);
    keypad(stdscr(), true);

    init_pair(1, COLOR_WHITE, COLOR_BLUE);
    init_pair(2, COLOR_BLACK, COLOR_YELLOW);
    init_pair(3, COLOR_YELLOW, COLOR_BLUE);
    init_pair(4, COLOR_WHITE, COLOR_MAGENTA);
    init_pair(5, COLOR_BLACK, COLOR_WHITE);
    init_pair(6, COLOR_BLACK, COLOR_YELLOW);
}
