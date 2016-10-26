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

use std::cell::RefMut;
use unicode_width::UnicodeWidthStr;

use ncurses::*;

use core::line::Line;
use core::buffer::BufferLines;
use utils::ansi_decoder::{Component, Style};
use ui::frame::{Frame, RenderedLine, NORMAL_HIGHLIGHT_COLOR, CURRENT_HIGHLIGHT_COLOR};
use ui::color::ColorPair;
use ui::content::{Content, State as ContentState};
use ui::search::{Query, Highlight};

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
    buffer_lines: &'a BufferLines<'a>
}

impl<'a> LinesPrinter<'a> {
    pub fn new(frame: &'a mut Frame, lines: &'a BufferLines<'a>) -> LinesPrinter<'a> {
        LinesPrinter {
            frame: frame,
            height: 0,
            buffer_lines: lines
        }
    }

    pub fn draw(&mut self) {
        if let Some(ref query) = self.buffer_lines.query {
            self.handle_print_with_search(query);
        } else {
            self.handle_print();
        }
    }

    fn handle_print(&mut self) {
        self.frame.reset();
        self.height = 0;

        for line in self.buffer_lines {
            let actual_height = self.frame.content.calculate_height_change(|| {
                line.print(&self.frame.content);
            });

            self.height += actual_height;
            self.frame.create_rendered_line(actual_height as usize, 0);
        }
    }

    fn handle_print_with_search(&mut self, query: &Query) {
        if query.highlight == Highlight::FirstVisibleOrLast {
            self.frame.reset();
            self.height = 0;

            for line in self.buffer_lines {
                let actual_height = self.frame.content.calculate_height_change(|| {
                    line.print(&self.frame.content);
                });

                let is_match = query.filter_mode || line.contains(&query.text);
                let mut found_matches = 0;
                if is_match {
                    self.frame.navigation.search.matches_found = true;
                    let highlighter = LineHighlighter::new(self.frame, line, NORMAL_HIGHLIGHT_COLOR);
                    found_matches = highlighter.print(&query.text, self.height, actual_height);
                }

                self.height += actual_height;
                self.frame.create_rendered_line(actual_height as usize, found_matches);
            }

            if self.frame.navigation.search.matches_found {
                self.update_current_and_highlight_item(query);
            }
        } else if self.frame.navigation.search.matches_found {
            self.highlight_current_item(&query.text, NORMAL_HIGHLIGHT_COLOR);
            self.update_current_and_highlight_item(query);
        }
    }

    fn update_current_and_highlight_item(&self, query: &Query) {
        let state = self.frame.content.state.borrow_mut();
        HighlightState::new(state, &self.frame.rendered_lines).update(&query.highlight);

        self.highlight_current_item(&query.text, CURRENT_HIGHLIGHT_COLOR);
    }

    fn highlight_current_item(&self, text: &str, color: i16) {
        let state = self.frame.content.state.borrow();
        let line = self.buffer_lines
            .into_iter()
            .skip(state.highlighted_line)
            .next()
            .unwrap();
        let accumulated_height = self.frame.rendered_lines
            .iter()
            .take(state.highlighted_line)
            .map(|line| line.height)
            .sum::<usize>();
        let highlighter = LineHighlighter::new(self.frame, line, color);
        highlighter.print_single_match(text, state.highlighted_match, accumulated_height as i32);
    }
}

struct LineHighlighter<'a> {
    frame: &'a Frame,
    line: &'a Line,
    color_pair_id: i16
}

impl<'a> LineHighlighter<'a> {
    fn new(frame: &'a Frame, line: &'a Line, color_pair_id: i16) -> LineHighlighter<'a> {
        LineHighlighter {
            frame: frame,
            line: line,
            color_pair_id: color_pair_id
        }
    }

    fn print(&self, text: &str, accumulated_height: i32, line_height: i32) -> usize {
        let matches = &self.line.matches_for(text);

        for &(offset_x, value) in matches {
            self.handle_match(offset_x as i32, accumulated_height, value);
        }

        wmove(self.frame.content.window, accumulated_height + line_height, 0);

        matches.len()
    }

    fn print_single_match(&self, text: &str, index: usize, offset_y: i32) {
        let (offset_x, value) = self.line.matches_for(text)[index];
        self.handle_match(offset_x as i32, offset_y, value);
    }

    fn handle_match(&self, mut offset_x: i32, mut offset_y: i32, value: &str) {
        if offset_x > self.frame.width {
            offset_x = self.line.content_without_ansi.split_at(offset_x as usize).0.width() as i32;
            offset_y += offset_x / self.frame.width;
            offset_x %= self.frame.width;
        }
        wattron(self.frame.content.window, COLOR_PAIR(self.color_pair_id));
        mvwprintw(self.frame.content.window, offset_y, offset_x, value);
        wattroff(self.frame.content.window, COLOR_PAIR(self.color_pair_id));
    }
}

struct HighlightState<'a> {
    state: RefMut<'a, ContentState>,
    rendered_lines: &'a [RenderedLine]
}

impl<'a> HighlightState<'a> {
    fn new(state: RefMut<'a, ContentState>, rendered_lines: &'a [RenderedLine]) -> HighlightState<'a> {
        HighlightState {
            state: state,
            rendered_lines: rendered_lines,
        }
    }

    fn update(&mut self, highlight: &Highlight) {
        match *highlight {
            Highlight::FirstVisibleOrLast => self.handle_first_visible_or_last(),
            Highlight::Next => self.handle_next(),
            Highlight::Previous => self.handle_previous()
        }
    }

    fn handle_first_visible_or_last(&mut self) {
        // TODO: Search lines in viewport first, fallback to current
        let rendered_line_with_index = self.rendered_lines
            .iter()
            .rev()
            .enumerate()
            .find(|&index_and_line| index_and_line.1.found_matches > 0)
            .unwrap();

        self.state.highlighted_line = self.rendered_lines.len() - rendered_line_with_index.0 - 1;
        self.state.highlighted_match = rendered_line_with_index.1.found_matches - 1;
    }

    fn handle_next(&mut self) {
        let rendered_line = &self.rendered_lines[self.state.highlighted_line];
        if self.state.highlighted_match < rendered_line.found_matches - 1 {
            self.state.highlighted_match += 1;
        } else {
            let rendered_line_with_index_opt = self.rendered_lines
                .iter()
                .enumerate()
                .skip(self.state.highlighted_line + 1)
                .find(|&index_and_line| index_and_line.1.found_matches > 0);
            if let Some(rendered_line_with_index) = rendered_line_with_index_opt {
                self.state.highlighted_line = rendered_line_with_index.0;
                self.state.highlighted_match = 0;
            }
        }
    }

    fn handle_previous(&mut self) {
        if self.state.highlighted_match > 0 {
            self.state.highlighted_match -= 1;
        } else if self.state.highlighted_line > 0 {
            let rendered_line_with_index_opt = self.rendered_lines
                .iter()
                .enumerate()
                .rev()
                .skip(self.rendered_lines.len() - self.state.highlighted_line)
                .find(|&index_and_line| index_and_line.1.found_matches > 0);
            if let Some(rendered_line_with_index) = rendered_line_with_index_opt {
                self.state.highlighted_line = rendered_line_with_index.0;
                self.state.highlighted_match = rendered_line_with_index.1.found_matches - 1;
            }
        }
    }
}
