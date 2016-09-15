use ncurses::*;

pub struct Search {
    pub window: WINDOW,
    panel: PANEL
}

impl Search {
    pub fn new(position_x: i32, position_y: i32) -> Search {
        let window = newwin(0, 0, position_x, position_y);

        Search {
            window: window,
            panel: new_panel(window)
        }
    }

    pub fn render(&self, _: u64, background: u64) {
        wprintw(self.window, &format!("Hello world"));
        refresh();
        wbkgd(self.window, background);
        wrefresh(self.window);
    }

    pub fn show(&self) {
        show_panel(self.panel);
    }

    pub fn hide(&self) {
        hide_panel(self.panel);
    }
}
