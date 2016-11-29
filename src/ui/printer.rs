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

use core::line::Line;
use core::buffer::BufferLines;
use utils::ansi_decoder::{Component, Style};
use ui::frame::{Frame, NORMAL_HIGHLIGHT_COLOR, CURRENT_HIGHLIGHT_COLOR};
use ui::color::ColorPair;
use ui::content::Content;
use ui::search::Query;
use ui::highlighter::{Highlight, LineHighlighter, State as HighlightState};

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
                waddch(content.window, '\n' as u32);
            }
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
            }
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
            }
            Style::Color(foreground, background) => {
                let color = ColorPair::from_options(foreground,
                                                    background,
                                                    state.foreground,
                                                    state.background);
                wattron(content.window, color.to_attr());

                state.foreground = color.foreground;
                state.background = color.background;
            }
            Style::Reset => {
                for (_, prop) in state.attributes.drain(..) {
                    wattroff(content.window, prop());
                }

                wattron(content.window, ColorPair::default().to_attr());
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct Viewport {
    pub reverse_index: usize,
    pub visible_height: usize,
}

impl Viewport {
    fn new(reverse_index: usize, visible_height: usize) -> Viewport {
        Viewport {
            reverse_index: reverse_index,
            visible_height: visible_height,
        }
    }

    pub fn limit(&self) -> usize {
        self.reverse_index + self.visible_height
    }
}

pub struct LinesPrinter<'a> {
    frame: &'a mut Frame,
    height: i32,
    buffer_lines: &'a BufferLines<'a>,
    query: Option<Query>,
}

impl<'a> LinesPrinter<'a> {
    pub fn new(frame: &'a mut Frame,
               lines: &'a BufferLines<'a>,
               query: Option<Query>)
               -> LinesPrinter<'a> {
        LinesPrinter {
            frame: frame,
            height: 0,
            buffer_lines: lines,
            query: query,
        }
    }

    pub fn draw(&mut self) {
        if self.query.is_some() {
            if self.query.as_ref().unwrap().filter {
                self.handle_filter()
            } else {
                if self.frame.initial_rendered_lines.is_some() {
                    self.handle_print();
                }
                self.handle_search();
            }
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
            self.frame.rendered_lines.create(line.clone(), actual_height, None);
        }
    }

    fn handle_search(&mut self) {
        let query = self.query.as_ref().unwrap();

        if query.highlight == Highlight::VisibleOrLast || query.highlight == Highlight::Current {
            self.frame.navigation.search.matches_found = false;
            self.height = 0;

            for rendered_line in self.frame
                .rendered_lines
                .entries
                .iter_mut() {
                if rendered_line.search(&query.text,
                                        &self.frame.content,
                                        self.frame.width,
                                        self.height) {
                    self.frame.navigation.search.matches_found = true;
                }

                self.height += rendered_line.height;
            }
            if query.highlight == Highlight::Current && self.highlight_doesnt_require_update() {
                self.highlight_current_item(&query.text, CURRENT_HIGHLIGHT_COLOR);
            } else if self.frame.navigation.search.matches_found {
                self.update_current_and_highlight_item();
            }
        } else if self.frame.navigation.search.matches_found {
            self.highlight_current_item(&query.text, NORMAL_HIGHLIGHT_COLOR);
            self.update_current_and_highlight_item();
        }
    }

    fn handle_filter(&mut self) {
        let query = self.query.as_ref().unwrap();

        if query.highlight == Highlight::VisibleOrLast || query.highlight == Highlight::Current {
            self.frame.content.clear();
            self.height = 0;

            let mut filtered_rendered_lines = self.frame
                .initial_rendered_lines
                .as_mut()
                .unwrap_or(&mut self.frame.rendered_lines)
                .matching(&query.text);
            self.frame.navigation.search.matches_found = !filtered_rendered_lines.is_empty();

            for rendered_line in filtered_rendered_lines.entries.iter_mut() {
                rendered_line.print(&self.frame.content, self.height);
                rendered_line.found_matches = rendered_line.highlight(&query.text,
                                                                      &self.frame.content,
                                                                      self.frame.width,
                                                                      self.height);

                self.height += rendered_line.height;
            }

            self.frame.replace_rendered_lines(filtered_rendered_lines);

            if query.highlight == Highlight::Current && self.highlight_doesnt_require_update() {
                self.highlight_current_item(&query.text, CURRENT_HIGHLIGHT_COLOR);
            } else if self.frame.navigation.search.matches_found {
                self.update_current_and_highlight_item();
            }
        } else if self.frame.navigation.search.matches_found {
            self.highlight_current_item(&query.text, NORMAL_HIGHLIGHT_COLOR);
            self.update_current_and_highlight_item();
        }
    }

    fn update_current_and_highlight_item(&self) {
        let query = self.query.as_ref().unwrap();
        let viewport = Viewport::new(self.buffer_lines.buffer.reverse_index.get(),
                                     self.frame.content_height() as usize);

        HighlightState::new(self.frame.content.state.borrow_mut(),
                            &self.frame.rendered_lines,
                            viewport)
            .update(&query.highlight);
        self.highlight_current_item(&query.text, CURRENT_HIGHLIGHT_COLOR);

        let matched_line = self.frame.content.highlighted_line();
        if !self.frame.rendered_lines.is_match_in_viewport(matched_line, viewport) {
            self.update_scroll_position();
        }
    }

    fn highlight_current_item(&self, text: &str, color: i16) {
        let state = self.frame.content.state.borrow();
        let line = &self.frame.rendered_lines[state.highlighted_line].line;

        let accumulated_height = self.frame
            .rendered_lines
            .height_up_to_index(state.highlighted_line);
        let highlighter =
            LineHighlighter::new(self.frame.content.window, line, self.frame.width, color);
        highlighter.print_single_match(text, state.highlighted_match, accumulated_height);
    }

    fn update_scroll_position(&self) {
        let state = self.frame.content.state.borrow();
        let index = self.frame
            .rendered_lines
            .buffer_reverse_index(state.highlighted_line, state.highlighted_match);
        self.buffer_lines
            .buffer
            .set_reverse_index(index - self.frame.height / 2, self.frame.max_scroll_value());
    }

    fn highlight_doesnt_require_update(&self) -> bool {
        let state = self.frame.content.state.borrow();
        self.frame.rendered_lines[state.highlighted_line].found_matches.is_some()
    }
}
