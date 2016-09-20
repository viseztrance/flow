use std::cell::RefCell;

use ncurses::*;

use frontend::readline;

static OPTIONS_WIDTH: i32 = 45;

pub struct Search {
    pub window: WINDOW,
    pub options: Options,
    pub input: Input,
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

    pub fn process_input(&mut self, keys: Vec<i32>) {
        self.input.process(keys);
    }

    pub fn find_next_match(&self) {
        // unimplemented!();
    }

    pub fn find_previous_match(&self) {
        // unimplemented!();
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
        self.render();
        show_panel(self.panel);
    }

    pub fn hide(&self) {
        hide_panel(self.panel);
        self.input.reset();
    }
}

pub struct Input {
    pub window: WINDOW,
    text: RefCell<Option<String>>
}

impl Input {
    fn new(parent_window: WINDOW) -> Input {
        let window = derwin(parent_window, 1, COLS - OPTIONS_WIDTH, 0, 1);
        syncok(window, true);

        Input {
            window: window,
            text: RefCell::new(None)
        }
    }

    fn render(&self) {
        wclear(self.window);
        wbkgd(self.window, COLOR_PAIR(1));
        self.prepare_view();
        wattron(self.window, COLOR_PAIR(1));
    }

    fn process(&self, keys: Vec<i32>) {
        for key in keys {
            readline::forward_input(key);
        }
        *self.text.borrow_mut() = Some(readline::read_buffer().to_string());
        self.prepare_view();
    }

    fn reset(&self) {
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        readline::update_buffer("");
        *self.text.borrow_mut() = None;
    }

    fn prepare_view(&self) {
        match *self.text.borrow_mut() {
            Some(_) => {
                curs_set(CURSOR_VISIBILITY::CURSOR_VERY_VISIBLE);
            },
            None => {
                curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
                wprintw(self.window, "Type to search …");
            }
        }
    }
}

pub struct Options {
    pub window: WINDOW,
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
        readline::handle_redisplay();
        wbkgd(self.window, COLOR_PAIR(1));

        self.print_navigation();
        self.print_highlight();
        self.print_filter();
    }

    fn print_navigation(&self) {
        wprintw(self.window, " / ");
        self.make_shortcut('N');
        wprintw(self.window, "ext / ");
        self.make_shortcut('P');
        wprintw(self.window, "rev");
    }

    fn print_highlight(&self) {
        wprintw(self.window, " / ");
        self.mark_as_active(self.highlight_mode, || {
            wprintw(self.window, "Highlight ");
            self.make_shortcut('A');
            wprintw(self.window, "ll");
        });
    }

    fn print_filter(&self) {
        wprintw(self.window, " / ");
        self.mark_as_active(self.filter_mode, || {
            wprintw(self.window, "Filter ");
            self.make_shortcut('M');
            wprintw(self.window, "ode");
        });
    }

    fn make_shortcut(&self, letter: char) {
        wattron(self.window, A_UNDERLINE());
        waddch(self.window, letter as u64);
        wattroff(self.window, A_UNDERLINE());
    }

    fn mark_as_active<F>(&self, active: bool, callback: F) where F : Fn() {
        if active {
            wattron(self.window, COLOR_PAIR(2));
        }
        callback();
        if active {
            wattroff(self.window, COLOR_PAIR(1));
        }
    }
}
