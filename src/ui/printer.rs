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
use ui::frame::{Frame, NORMAL_HIGHLIGHT_COLOR, CURRENT_HIGHLIGHT_COLOR};
use ui::color::ColorPair;
use ui::content::{Content, State as ContentState};
use ui::search::{Query, Highlight};
use ui::rendered_line::RenderedLineCollection;

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
}

impl<'a> LinesPrinter<'a> {
    pub fn new(frame: &'a mut Frame, lines: &'a BufferLines<'a>) -> LinesPrinter<'a> {
        LinesPrinter {
            frame: frame,
            height: 0,
            buffer_lines: lines,
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
            self.frame.rendered_lines.create(actual_height, None);
        }
    }

    fn handle_print_with_search(&mut self, query: &Query) {
        if query.highlight == Highlight::VisibleOrLast {
            self.frame.reset();
            self.height = 0;

            for line in self.buffer_lines {
                let actual_height = self.frame.content.calculate_height_change(|| {
                    line.print(&self.frame.content);
                });

                let is_match = query.filter_mode || line.contains(&query.text);
                let mut found_matches = None;
                if is_match {
                    self.frame.navigation.search.matches_found = true;
                    let highlighter =
                        LineHighlighter::new(self.frame, line, NORMAL_HIGHLIGHT_COLOR);
                    found_matches =
                        Some(highlighter.print(&query.text, self.height, actual_height));
                }

                self.height += actual_height;
                self.frame.rendered_lines.create(actual_height, found_matches);
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
        let line = &self.buffer_lines[state.highlighted_line];

        let accumulated_height = self.frame
            .rendered_lines
            .height_up_to_index(state.highlighted_line);
        let highlighter = LineHighlighter::new(self.frame, line, color);
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
}

struct LineHighlighter<'a> {
    line: &'a Line,
    window: WINDOW,
    container_width: i32,
    color_pair_id: i16,
}

impl<'a> LineHighlighter<'a> {
    fn new(frame: &'a Frame, line: &'a Line, color_pair_id: i16) -> LineHighlighter<'a> {
        LineHighlighter {
            line: line,
            window: frame.content.window,
            container_width: frame.width,
            color_pair_id: color_pair_id,
        }
    }

    fn print(&self, text: &str, accumulated_height: i32, line_height: i32) -> Vec<usize> {
        let mut locations = vec![];

        let matches = &self.line.matches_for(text);

        for &(offset_x, value) in matches {
            let location = self.handle_match(offset_x as i32, accumulated_height, value);
            locations.push(location);
        }

        wmove(self.window, accumulated_height + line_height, 0);

        locations
    }

    fn print_single_match(&self, text: &str, index: usize, offset_y: i32) {
        let (offset_x, value) = self.line.matches_for(text)[index];
        self.handle_match(offset_x as i32, offset_y, value);
    }

    fn handle_match(&self, mut offset_x: i32, mut offset_y: i32, value: &str) -> usize {
        let initial_offset_y = offset_y;

        offset_x = self.line.content_without_ansi.split_at(offset_x as usize).0.width() as i32;
        offset_y += offset_x / self.container_width;
        offset_x %= self.container_width;

        wattron(self.window, COLOR_PAIR(self.color_pair_id));
        mvwprintw(self.window, offset_y, offset_x, value);
        wattroff(self.window, COLOR_PAIR(self.color_pair_id));

        (offset_y - initial_offset_y) as usize
    }
}

struct HighlightState<'a> {
    state: RefMut<'a, ContentState>,
    rendered_lines: &'a RenderedLineCollection,
    viewport: Viewport,
}

impl<'a> HighlightState<'a> {
    fn new(state: RefMut<'a, ContentState>,
           rendered_lines: &'a RenderedLineCollection,
           viewport: Viewport)
           -> HighlightState<'a> {
        HighlightState {
            state: state,
            rendered_lines: rendered_lines,
            viewport: viewport,
        }
    }

    fn update(&mut self, highlight: &Highlight) {
        match *highlight {
            Highlight::VisibleOrLast => self.handle_visible_or_last(),
            Highlight::Next => self.handle_next(),
            Highlight::Previous => self.handle_previous(),
        }
    }

    fn handle_visible_or_last(&mut self) {
        let matched_line = self.rendered_lines
            .viewport_match(&self.viewport)
            .unwrap_or(self.rendered_lines.first_match());

        self.state.highlighted_line = self.rendered_lines.len() - matched_line.line - 1;
        self.state.highlighted_match = matched_line.match_index;
    }

    fn handle_next(&mut self) {
        let rendered_line = &self.rendered_lines[self.state.highlighted_line];
        if self.state.highlighted_match < rendered_line.match_count() - 1 {
            self.state.highlighted_match += 1;
        } else {
            let matched_line_opt = self.rendered_lines
                .next_match(self.state.highlighted_line);

            if let Some(matched_line) = matched_line_opt {
                self.state.highlighted_line = matched_line.line;
                self.state.highlighted_match = 0;
            }
        }
    }

    fn handle_previous(&mut self) {
        if self.state.highlighted_match > 0 {
            self.state.highlighted_match -= 1;
        } else if self.state.highlighted_line > 0 {
            let matched_line_opt = self.rendered_lines
                .previous_match(self.state.highlighted_line);

            if let Some(matched_line) = matched_line_opt {
                self.state.highlighted_line = matched_line.line;
                self.state.highlighted_match = matched_line.match_index;
            }
        }
    }
}
