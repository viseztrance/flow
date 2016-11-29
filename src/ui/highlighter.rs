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
use ui::printer::Viewport;
use ui::content::State as ContentState;
use ui::rendered_line::RenderedLineCollection;

#[derive(PartialEq)]
pub enum Highlight {
    VisibleOrLast,
    Next,
    Previous,
    Current,
}

pub struct LineHighlighter<'a> {
    line: &'a Line,
    window: WINDOW,
    container_width: i32,
    color_pair_id: i16,
}

impl<'a> LineHighlighter<'a> {
    pub fn new(window: WINDOW,
               line: &'a Line,
               container_width: i32,
               color_pair_id: i16)
               -> LineHighlighter<'a> {
        LineHighlighter {
            line: line,
            window: window,
            container_width: container_width,
            color_pair_id: color_pair_id,
        }
    }

    pub fn print(&self, text: &str, accumulated_height: i32, line_height: i32) -> Vec<usize> {
        let mut locations = vec![];

        let matches = &self.line.matches_for(text);

        for &(offset_x, value) in matches {
            let location = self.handle_match(offset_x as i32, accumulated_height, value);
            locations.push(location);
        }

        wmove(self.window, accumulated_height + line_height, 0);

        locations
    }

    pub fn print_single_match(&self, text: &str, index: usize, offset_y: i32) {
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

pub struct State<'a> {
    state: RefMut<'a, ContentState>,
    rendered_lines: &'a RenderedLineCollection,
    viewport: Viewport,
}

impl<'a> State<'a> {
    pub fn new(state: RefMut<'a, ContentState>,
               rendered_lines: &'a RenderedLineCollection,
               viewport: Viewport)
               -> State<'a> {
        State {
            state: state,
            rendered_lines: rendered_lines,
            viewport: viewport,
        }
    }

    pub fn update(&mut self, highlight: &Highlight) {
        match *highlight {
            Highlight::VisibleOrLast | Highlight::Current => self.handle_visible_or_last(),
            Highlight::Next => self.handle_next(),
            Highlight::Previous => self.handle_previous(),
        }
    }

    fn handle_visible_or_last(&mut self) {
        let matched_line = self.rendered_lines
            .viewport_match(&self.viewport)
            .unwrap_or(self.rendered_lines.last_match());

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
