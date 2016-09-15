use std::sync::{Arc, Mutex};
use std::cell::RefCell;

use settings::Settings;
use ui::ui::{Ui, Event, Direction};
use flow::line::LineCollection;
use flow::buffer::{Buffer, BufferCollection, ScrollState};

pub struct Flow {
    ui: Ui,
    lines: LineCollection,
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
            lines: LineCollection::new(settings.values.max_lines_count),
            buffer_collection: BufferCollection::from_filters(settings.config_file.filters)
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
                Event::Resize => self.resize(),
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

    fn resize(&mut self) {
        self.ui.resize();
        self.current_buffer().borrow_mut().reset_reverse_index();
        self.reset_view();
    }

    fn append_incoming_lines(&mut self, pending_lines: Vec<String>) {
        let initial_screen_lines = self.ui.screen_lines;

        self.lines.extend(pending_lines);

        self.reset_view();
        if self.current_buffer().borrow().is_scrolled() {
            let offset = self.ui.screen_lines - initial_screen_lines;
            self.scroll(offset);
        }

        self.lines.clear_excess();
    }

    fn reset_view(&mut self) {
        self.ui.content.clear();
        let lines_iter = self.current_buffer().borrow().parse(&self.lines);
        self.ui.print(lines_iter);
        self.ui.scroll(self.current_buffer().borrow().reverse_index as i32);
    }

    fn current_buffer(&self) -> &RefCell<Buffer> {
        self.buffer_collection.selected_item()
    }
}
