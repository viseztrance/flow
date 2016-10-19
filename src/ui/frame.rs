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

use unicode_width::UnicodeWidthStr;

use ncurses::*;

use core::line::Line;
use core::buffer::BufferLines;
use ui::readline;
use ui::color;
use ui::input::read_key;
use ui::event::{EventBuilder, Event};
use ui::navigation::Navigation;
use ui::content::Content;
use ui::printer::Print;
use ui::search::Query;

pub struct Frame {
    pub width: i32,
    pub height: i32,
    pub rendered_lines_height: i32,
    pub navigation: Navigation,
    pub content: Content
}

impl Frame {
    pub fn new(menu_item_names: &Vec<String>) -> Frame {
        ::std::env::set_var("ESCDELAY", "25");
        setlocale(LcCategory::all, ""); // Must be set *before* ncurses init

        readline::init();

        initscr();
        start_color();
        use_default_colors();
        cbreak();
        noecho();
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        halfdelay(1);
        keypad(stdscr, true);

        init_pair(1, COLOR_WHITE, COLOR_BLUE);
        init_pair(2, COLOR_BLACK, COLOR_YELLOW);
        init_pair(3, COLOR_YELLOW, COLOR_BLUE);
        init_pair(4, COLOR_WHITE, COLOR_MAGENTA);
        color::generate_pairs();

        Frame {
            width: COLS,
            height: LINES,
            rendered_lines_height: 0,
            navigation: Navigation::new(LINES - 1, 0, menu_item_names),
            content: Content::new(COLS)
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
        getmaxyx(stdscr, &mut self.height, &mut self.width);

        self.content.resize(self.width);
        self.navigation.resize(0, self.height - 1);
    }

    pub fn print<'a>(&mut self, buffer_lines: &mut BufferLines, query: Option<Query>) {
        buffer_lines.set_context(self.width as usize, query);

        LinesPrinter::new(self, buffer_lines).draw();
        self.scroll(buffer_lines.buffer.reverse_index as i32);
    }

    pub fn scroll(&self, reversed_offset: i32) {
        let offset = self.rendered_lines_height - self.height + 1 - reversed_offset;
        prefresh(self.content.window, offset, 0, 0, 0, self.height - 2, self.width);
    }

    pub fn watch(&self) -> Event {
        let (input, key) = read_key();
        EventBuilder::new(input, key).construct(&self.navigation.state)
    }
}

struct LinesPrinter<'a> {
    frame: &'a mut Frame,
    height: i32,
    buffer_lines: &'a BufferLines<'a>,
    rendered_lines: Vec<RenderedLine>
}

impl<'a> LinesPrinter<'a> {
    pub fn new(frame: &'a mut Frame, lines: &'a BufferLines<'a>) -> LinesPrinter<'a> {
        LinesPrinter {
            frame: frame,
            height: 0,
            rendered_lines: vec![],
            buffer_lines: lines
        }
    }

    pub fn draw(&mut self) {
        self.frame.content.clear();
        self.frame.navigation.search.matches_found = false;
        self.height = 0;

        if let Some(ref query) = self.buffer_lines.query {
            self.handle_print_with_search(query);
        } else {
            self.handle_print();
        }

        self.frame.rendered_lines_height = self.rendered_lines_height() as i32;
    }

    fn rendered_lines_height(&self) -> usize {
        self.rendered_lines.iter().map(|line| line.height).sum()
    }

    fn handle_print(&mut self) {
        for line in self.buffer_lines {
            let actual_height = self.frame.content.calculate_height_change(|| {
                line.print(&self.frame.content);
            });

            self.height += actual_height;
            self.rendered_lines.push(RenderedLine::new(actual_height as usize, false));
        }
    }

    fn handle_print_with_search(&mut self, query: &Query) {
        for line in self.buffer_lines {
            let actual_height = self.frame.content.calculate_height_change(|| {
                line.print(&self.frame.content);
            });

            let is_match = query.filter_mode || line.content_without_ansi.contains(&query.text);
            if is_match {
                self.frame.navigation.search.matches_found = true;
                self.highlight(line, query, actual_height);
            }

            self.height += actual_height;
            self.rendered_lines.push(RenderedLine::new(actual_height as usize, is_match));
        }
    }

    fn highlight(&self, line: &Line, query: &Query, height: i32) {
        let matches: Vec<_> = line.content_without_ansi.match_indices(&query.text).collect();
        for (i, value) in matches {
            let mut offset_x = i as i32;
            let mut offset_y  = self.height;
            if offset_x > self.frame.width {
                offset_x = line.content_without_ansi.split_at(i).0.width() as i32;
                offset_y = (offset_x / self.frame.width) + offset_y;
                offset_x = offset_x % self.frame.width;
            }
            wattron(self.frame.content.window, A_STANDOUT());
            mvwprintw(self.frame.content.window, offset_y, offset_x, value);
            wattroff(self.frame.content.window, A_STANDOUT());
        }
        wmove(self.frame.content.window, self.height + height, 0);
    }
}

pub struct RenderedLine {
    height: usize,
    highlighted: bool
}

impl RenderedLine {
    fn new(height: usize, highlighted: bool) -> RenderedLine {
        RenderedLine {
            height: height,
            highlighted: highlighted
        }
    }
}
