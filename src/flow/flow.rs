use std::sync::{Arc, Mutex};
use std::cell::RefCell;
use std::collections::VecDeque;

use settings::{Settings, SettingsValues};
use flow::ui::{Ui, Event, Direction};
use flow::buffer::{Buffer, BufferCollection, ScrollState};

pub struct Flow {
    ui: Ui,
    lines: VecDeque<String>,
    settings_values: SettingsValues,
    buffer_collection: BufferCollection
}

impl Flow {
    pub fn new(settings: Settings) -> Flow {
        let tab_names = settings
            .config_file
            .filters
            .iter()
            .map(|tab| tab.name.clone())
            .collect();

        Flow {
            ui: Ui::new(&tab_names),
            lines: VecDeque::new(),
            buffer_collection: BufferCollection::from_filters(settings.config_file.filters),
            settings_values: settings.values,
        }
    }

    pub fn render(&self) {
        self.ui.render();
    }

    pub fn terminate(&self) {
        self.ui.destroy();
    }

    pub fn process(&mut self, lines: Arc<Mutex<Vec<String>>>) {
        loop {
            match self.ui.watch() {
                Event::SelectMenuItem(direction) => self.select_menu_item(direction),
                Event::ScrollContents(value) => self.scroll(value),
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
                self.ui.select_left_menu_item();
                self.buffer_collection.select_previous();
            },
            Direction::Right => {
                self.ui.select_right_menu_item();
                self.buffer_collection.select_next();
            }
        };
        self.reset_view();
    }

    fn scroll(&mut self, value: i32) {
        let result = self.current_buffer()
            .borrow_mut()
            .adjust_reverse_index(value, &self.lines);

        if result == ScrollState::Changed {
            let offset = self.current_buffer().borrow().reverse_index as i32;
            self.ui.scroll(offset);
        }
    }

    fn append_incoming_lines(&mut self, pending_lines: Vec<String>) {
        let initial_screen_lines = self.ui.screen_lines;

        self.lines.extend(pending_lines);

        self.reset_view();
        if self.current_buffer().borrow().is_scrolled() {
            let offset = self.ui.screen_lines - initial_screen_lines;
            self.scroll(offset);
        }

        self.clear_excess_lines();
    }

    fn reset_view(&mut self) {
        self.ui.clear();
        let lines_iter = self.current_buffer().borrow().parse(&self.lines);
        self.ui.print(lines_iter);
        self.ui.scroll(self.current_buffer().borrow().reverse_index as i32);
    }

    fn clear_excess_lines(&mut self) {
        while self.lines.len() > self.settings_values.max_lines_count {
            self.lines.pop_front();
        }
    }

    fn current_buffer(&self) -> &RefCell<Buffer> {
        self.buffer_collection.selected_item()
    }
}
