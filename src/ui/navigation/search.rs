use ncurses::*;

static OPTIONS_WIDTH: i32 = 45;

pub struct Search {
    pub window: WINDOW,
    options: Options,
    panel: PANEL
}

impl Search {
    pub fn new(position_x: i32, position_y: i32) -> Search {
        let window = newwin(0, 0, position_x, position_y);

        Search {
            window: window,
            options: Options::new(window),
            panel: new_panel(window)
        }
    }

    pub fn render(&self, _: u64, background: u64) {
        wprintw(self.window, &format!("Hello world"));
        refresh();
        wbkgd(self.window, background);
        self.options.render(background);
        wrefresh(self.window);
    }

    pub fn find_next_match(&self) {
        // Highlight match
    }

    pub fn find_previous_match(&self) {
        // Highlight match
    }

    pub fn toggle_highlight_mode(&mut self) {
        self.options.highlight_mode = !self.options.highlight_mode;
    }

    pub fn toggle_filter_mode(&mut self) {
        self.options.filter_mode = !self.options.filter_mode;
    }

    pub fn show(&self) {
        show_panel(self.panel);
    }

    pub fn hide(&self) {
        hide_panel(self.panel);
    }
}

struct Options {
    window: WINDOW,
    highlight_mode: bool,
    filter_mode: bool
}

impl Options {
    fn new(parent_window: WINDOW) -> Options {
        Options {
            window: derwin(parent_window, 1, OPTIONS_WIDTH, 0, COLS - OPTIONS_WIDTH),
            highlight_mode: false,
            filter_mode: false
        }
    }

    fn render(&self, background: u64) {
        wbkgd(self.window, background);

        self.print_navigation();
        self.print_highlight();
        self.print_filter();
    }

    fn print_navigation(&self) {
        wprintw(self.window, &format!(" / "));
        self.make_shortcut('N');
        wprintw(self.window, &format!("ext / "));
        self.make_shortcut('P');
        wprintw(self.window, &format!("rev"));
    }

    fn print_highlight(&self) {
        wprintw(self.window, &format!(" / "));
        self.mark_as_active(self.highlight_mode, || {
            wprintw(self.window, &format!("Highlight "));
            self.make_shortcut('A');
            wprintw(self.window, &format!("ll"));
        });
    }

    fn print_filter(&self) {
        wprintw(self.window, &format!(" / "));
        self.mark_as_active(self.filter_mode, || {
            self.make_shortcut('F');
            wprintw(self.window, &format!("ilter Mode"));
        });
    }

    fn make_shortcut(&self, letter: char) {
        wattron(self.window, A_BOLD() as i32);
        waddch(self.window, letter as u64);
        wattroff(self.window, A_BOLD() as i32);
    }

    fn mark_as_active<F>(&self, active: bool, callback: F) where F : Fn() {
        if active {
            wattron(self.window, COLOR_PAIR(1) as i32);
        }
        callback();
        if active {
            wattroff(self.window, COLOR_PAIR(2) as i32);
        }
    }
}
