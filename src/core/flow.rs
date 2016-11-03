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

use std::sync::{Arc, Mutex};
use std::sync::atomic::Ordering;

use ui::readline;
use utils::settings::Settings;
use ui::frame::Frame;
use ui::event::{Event, Direction, SearchAction, Offset};
use ui::navigation::State as NavigationState;
use ui::search::{State as QueryState, Highlight};

use core::runner::RUNNING;
use core::line::LineCollection;
use core::buffer::BufferCollection;
use ext::signal::{self, SIGQUIT};

pub struct Flow {
    frame: Frame,
    lines: LineCollection,
    buffer_collection: BufferCollection,
}

impl Flow {
    pub fn new(settings: Settings) -> Flow {
        let menu_item_names = settings.filters
            .iter()
            .map(|tab| tab.name.clone())
            .collect();

        Flow {
            frame: Frame::new(menu_item_names),
            lines: LineCollection::new(settings.max_lines_count),
            buffer_collection: BufferCollection::from_filters(settings.filters),
        }
    }

    pub fn render(&self) {
        self.frame.render();
    }

    pub fn terminate(&self) {
        self.frame.destroy();
    }

    pub fn process(&mut self, lines: Arc<Mutex<Vec<String>>>) {
        while running!() {
            match self.frame.watch() {
                Event::SelectMenuItem(direction) => self.select_menu_item(direction),
                Event::ScrollContents(offset) => self.scroll(offset),
                Event::Navigation(state) => {
                    if self.frame.navigation.change_state(state) {
                        match self.frame.navigation.state {
                            NavigationState::Search => readline::move_cursor(),
                            NavigationState::Menu => self.reset_view(),
                        }
                    }
                }
                Event::Search(action) => self.handle_search(action),
                Event::Resize => self.resize(),
                Event::Quit => self.quit(),
                _ => {
                    let mut mutex_guarded_lines = lines.lock().unwrap();
                    if !mutex_guarded_lines.is_empty() {
                        let pending_lines = mutex_guarded_lines.drain(..).collect();
                        self.append_incoming_lines(pending_lines);
                    }
                }
            };
        }
    }

    fn select_menu_item(&mut self, direction: Direction) {
        match direction {
            Direction::Left => {
                self.frame.select_left_menu_item();
                self.buffer_collection.select_previous();
            }
            Direction::Right => {
                self.frame.select_right_menu_item();
                self.buffer_collection.select_next();
            }
        };
        self.reset_view();
    }

    fn scroll(&mut self, offset: Offset) {
        let buffer = self.buffer_collection.selected_item();

        match offset {
            Offset::Line(value) => {
                buffer.increment_reverse_index(value, self.frame.max_scroll_value());
            }
            Offset::Viewport(value) => {
                buffer.increment_reverse_index(value * self.frame.height - 4,
                                               self.frame.max_scroll_value());
            }
            Offset::Top => {
                buffer.reverse_index.set(self.frame.max_scroll_value() as usize);
            }
            Offset::Bottom => {
                buffer.reset_reverse_index();
            }
        };

        self.frame.scroll(buffer.reverse_index.get() as i32);
    }

    fn handle_search(&mut self, action: SearchAction) {
        match action {
            SearchAction::ReadInput(keys) => {
                if self.frame.navigation.search.input_field.read(keys) == QueryState::Changed {
                    self.perform_search(Highlight::VisibleOrLast);
                }
            }
            SearchAction::FindNextMatch => {
                self.perform_search(Highlight::Next);
            }
            SearchAction::FindPreviousMatch => {
                self.perform_search(Highlight::Previous);
            }
            SearchAction::ToggleFilterMode => {
                self.frame.navigation.search.toggle_filter_mode();
                self.perform_search(Highlight::VisibleOrLast);
            }
        }
    }

    fn resize(&mut self) {
        self.frame.resize();
        self.reset_view_or_redo_search();
    }

    fn append_incoming_lines(&mut self, pending_lines: Vec<String>) {
        let count = pending_lines.len();
        self.lines.extend(pending_lines);

        if self.frame.navigation.state == NavigationState::Search {
            let mut state = self.frame.content.state.borrow_mut();
            let new_highlighted_line = state.highlighted_line as i32 - count as i32;
            if new_highlighted_line >= 0 {
                state.highlighted_line -= count;
            }
        }

        self.reset_view_or_redo_search();

        if self.buffer_collection.selected_item().is_scrolled() {
            let offset = self.frame.rendered_lines.last_lines_height(count);
            self.scroll(Offset::Line(offset));
        }
    }

    fn reset_view(&mut self) {
        let buffer = self.buffer_collection.selected_item();
        self.frame.print(&mut buffer.with_lines(&self.lines), None);
    }

    fn reset_view_or_redo_search(&mut self) {
        match self.frame.navigation.state {
            NavigationState::Search => self.perform_search(Highlight::Current),
            NavigationState::Menu => self.reset_view(),
        }
    }

    fn perform_search(&mut self, highlight: Highlight) {
        let buffer = self.buffer_collection.selected_item();
        let query = self.frame.navigation.search.build_query(highlight);
        self.frame.print(&mut buffer.with_lines(&self.lines), query);
        self.frame.navigation.search.render();
    }

    fn quit(&self) {
        unsafe {
            signal::raise(SIGQUIT);
        }
    }
}
