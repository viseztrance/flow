use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

use settings::{Settings, SettingsValues};
use flow::ui::{Ui, Key};
use flow::filter::FilterCollection;

pub struct Flow {
    ui: Ui,
    lines: VecDeque<String>,
    settings_values: SettingsValues,
    filter_collection: FilterCollection
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
            filter_collection: FilterCollection::new(settings.config_file.filters),
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
            match self.ui.read_input() {
                Key::Left => {
                    self.ui.select_left_menu_item();
                    self.filter_collection.select_left_item();
                    self.display();
                },
                Key::Right => {
                    self.ui.select_right_menu_item();
                    self.filter_collection.select_right_item();
                    self.display();
                },
                Key::Unknown => {
                    let mut mutex_guarded_lines = lines.lock().unwrap();
                    let pending_lines = mutex_guarded_lines.drain(..).collect();
                    self.append_and_display(pending_lines);
                }
            };
        }
    }

    pub fn display(&self) {
        self.ui.clear();
        self.filter_collection.selected_item().parse(&self.lines, |line| {
            self.ui.print(line)
        });
        self.ui.refresh();
    }

    pub fn append_and_display(&mut self, pending_lines: Vec<String>) {
        self.filter_collection.selected_item().parse(&pending_lines, |line| {
            self.ui.print(line)
        });
        self.ui.refresh();

        self.append(pending_lines);
    }

    fn append(&mut self, pending_lines: Vec<String>) {
        self.lines.extend(pending_lines);

        while self.lines.len() > self.settings_values.max_lines_count {
            self.lines.pop_front();
        }
    }
}
