use std::sync::{Arc, Mutex};
use std::cell::RefCell;
use std::collections::VecDeque;

use settings::{Settings, SettingsValues};
use flow::ui::{Ui, Event, Key};
use flow::buffer::{BufferCollection, Buffer};

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
                Event::SelectMenuItem(key) => {
                    match key {
                        Key::Left => {
                            self.ui.select_left_menu_item();
                            self.buffer_collection.select_left_item();
                        },
                        Key::Right => {
                            self.ui.select_right_menu_item();
                            self.buffer_collection.select_right_item();
                        },
                        _ => {}
                    };
                    self.display();
                },
                Event::ScrollContents(key) => {
                    match key {
                        Key::Up => {
                            self.current_buffer()
                                .borrow_mut()
                                .increment_reverse_index(1, &self.lines);
                        },
                        Key::Down => {
                            self.current_buffer()
                                .borrow_mut()
                                .decrement_reverse_index(1);
                        },
                        _ => {}
                    };
                    self.ui.scroll(self.current_buffer().borrow().reverse_index as i32);
                },
                _ => {
                    let mut mutex_guarded_lines = lines.lock().unwrap();
                    if !mutex_guarded_lines.is_empty() {
                        let pending_lines = mutex_guarded_lines.drain(..).collect();
                        self.append(pending_lines);
                        self.display();
                    }
                }
            };
        }
    }

    pub fn display(&mut self) {
        self.ui.clear();
        let lines_iter = self.current_buffer().borrow().parse(&self.lines);
        self.ui.print(lines_iter);
        self.ui.scroll(self.current_buffer().borrow().reverse_index as i32);
    }

    fn append(&mut self, pending_lines: Vec<String>) {
        self.lines.extend(pending_lines);

        while self.lines.len() > self.settings_values.max_lines_count {
            self.lines.pop_front();
        }
    }

    fn current_buffer(&self) -> &RefCell<Buffer> {
        self.buffer_collection.selected_item()
    }
}
