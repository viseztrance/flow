use ncurses::*;

static OPTIONS_WIDTH: i32 = 45;

pub struct Search {
    pub window: WINDOW,
    options: Options,
    input: Input,
    panel: PANEL
}

impl Search {
    pub fn new(position_x: i32, position_y: i32) -> Search {
        let window = newwin(0, 0, position_x, position_y);

        Search {
            window: window,
            options: Options::new(window),
            input: Input::new(window),
            panel: new_panel(window)
        }
    }

    pub fn render(&self) {
        wbkgd(self.window, COLOR_PAIR(1));
        self.input.render();
        self.options.render();
        wrefresh(self.window);
    }

    pub fn process_input(&self, value: char) {
        self.input.process(value);
    }

    pub fn find_next_match(&self) {
        unimplemented!();
    }

    pub fn find_previous_match(&self) {
        unimplemented!();
    }

    pub fn toggle_highlight_mode(&mut self) {
        self.options.highlight_mode = !self.options.highlight_mode;
        self.render();
    }

    pub fn toggle_filter_mode(&mut self) {
        self.options.filter_mode = !self.options.filter_mode;
        self.render();
    }

    pub fn show(&self) {
        show_panel(self.panel);
    }

    pub fn hide(&self) {
        hide_panel(self.panel);
    }
}

struct Input {
    window: WINDOW,
    text: Option<String>
}

impl Input {
    fn new(parent_window: WINDOW) -> Input {
        Input {
            window: derwin(parent_window, 1, COLS - OPTIONS_WIDTH, 0, 1),
            text: None
        }
    }

    fn render(&self) {
        wclear(self.window);
        wbkgd(self.window, COLOR_PAIR(1));
        match self.text {
            Some(ref value) => {
                wattron(self.window, COLOR_PAIR(3) as i32);
                wprintw(self.window, &value);
            },
            None => {
                wprintw(self.window, "Type to search …");
            }
        }
        wattron(self.window, COLOR_PAIR(1) as i32);
    }

    fn process(&self, value: char) {
        // display cursor
        // assume utf8 strings
        // readline support
        // empty strings becomes None
        // render after each change
        unimplemented!();
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

    fn render(&self) {
        wclear(self.window);
        wbkgd(self.window, COLOR_PAIR(1));

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
        wattron(self.window, A_UNDERLINE() as i32);
        waddch(self.window, letter as u64);
        wattroff(self.window, A_UNDERLINE() as i32);
    }

    fn mark_as_active<F>(&self, active: bool, callback: F) where F : Fn() {
        if active {
            wattron(self.window, COLOR_PAIR(2) as i32);
        }
        callback();
        if active {
            wattroff(self.window, COLOR_PAIR(1) as i32);
        }
    }
}
