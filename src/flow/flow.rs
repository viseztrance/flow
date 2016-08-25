use std::sync::{Arc, Mutex};

use flow::ui::{Ui, Key};

pub struct Flow {
    ui: Ui,
    lines: Vec<String>
}

impl Flow {
    pub fn new(buffers: Vec<&str>) -> Flow {
        Flow {
            ui: Ui::new(buffers),
            lines: vec![]
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
                    self.display();
                },
                Key::Right => {
                    self.ui.select_right_menu_item();
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
        self.ui.print(&self.lines);
    }

    pub fn append_and_display(&mut self, pending_lines: Vec<String>) {
        self.ui.print(&pending_lines);
        self.lines.extend(pending_lines);
    }
}
