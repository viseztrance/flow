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
use utils::ansi_decoder::{Component, Style};
use ui::frame::Frame;
use ui::color::ColorPair;
use ui::content::Content;
use ui::search::Query;

pub trait Print {
    fn print(&self, content: &Content);
}

impl Print for Line {
    fn print(&self, content: &Content) {
        match self.components {
            Some(ref value) => {
                for component in &value.items {
                    component.print(content);
                }
                waddch(content.window, '\n' as u64);
            },
            None => {
                wprintw(content.window, &format!("{}\n", self.content_without_ansi));
            }
        };
    }
}

impl Print for Component {
    fn print(&self, content: &Content) {
        match *self {
            Component::Style(value) => {
                value.print(content);
            },
            Component::Content(ref value) => {
                wprintw(content.window, value);
            }
        };
    }
}

impl Print for Style {
    fn print(&self, content: &Content) {
        let mut state = content.state.borrow_mut();

        match *self {
            Style::Attribute(id, prop, active) => {
                if active {
                    state.attributes.push((id, prop));
                    wattron(content.window, prop());
                } else {
                    wattroff(content.window, prop());
                    state.remove_attribute(id);
                }
            },
            Style::Color(foreground, background) => {
                let color = ColorPair::from_options(
                    foreground,
                    background,
                    state.foreground,
                    state.background
                );
                wattron(content.window, color.to_attr());

                state.foreground = color.foreground;
                state.background = color.background;
            },
            Style::Reset => {
                for (_, prop) in state.attributes.drain(..) {
                    wattroff(content.window, prop());
                }

                wattron(content.window, ColorPair::default().to_attr());
            }
        }
    }
}

pub struct LinesPrinter<'a> {
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
