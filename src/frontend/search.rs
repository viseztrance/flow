use std::cell::RefCell;

use ncurses::*;

use frontend::readline;

static OPTIONS_WIDTH: i32 = 45;

pub struct Query {
    pub text: String,
    pub highlight_mode: bool,
    pub filter_mode: bool
}

pub struct Search {
    pub window: WINDOW,
    pub options: Options,
    pub input_field: InputField,
    panel: PANEL
}

impl Search {
    pub fn new(position_x: i32, position_y: i32) -> Search {
        let window = newwin(0, 0, position_x, position_y);

        Search {
            window: window,
            options: Options::new(window),
            input_field: InputField::new(window),
            panel: new_panel(window)
        }
    }

    pub fn render(&self) {
        wbkgd(self.window, COLOR_PAIR(1));
        self.input_field.render();
        self.options.render();
        wrefresh(self.window);
    }

    pub fn build_query(&self) -> Query {
        let query = Query {
            text: self.input_field.text.borrow().clone(),
            highlight_mode: self.options.highlight_mode,
            filter_mode: self.options.filter_mode
        };
        query
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
        curs_set(CURSOR_VISIBILITY::CURSOR_VERY_VISIBLE);
        show_panel(self.panel);
        self.input_field.reset();
    }

    pub fn hide(&self) {
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        hide_panel(self.panel);
    }
}
#[derive(PartialEq)]
pub enum QueryState {
    Changed,
    Unchanged
}

pub struct InputField {
    pub window: WINDOW,
    text: RefCell<String>
}

impl InputField {
    fn new(parent_window: WINDOW) -> InputField {
        let window = derwin(parent_window, 1, COLS - OPTIONS_WIDTH, 0, 1);
        syncok(window, true);

        InputField {
            window: window,
            text: RefCell::new(String::new())
        }
    }

    fn render(&self) {
        wclear(self.window);
        wbkgd(self.window, COLOR_PAIR(1));
        wattron(self.window, COLOR_PAIR(1));
    }

    pub fn read(&self, keys: Vec<i32>) -> QueryState {
        for key in keys {
            readline::forward_input(key);
        }

        let pending_text = readline::read_buffer().to_string();
        let mut current_text = self.text.borrow_mut();

        if *current_text == pending_text {
            QueryState::Unchanged
        } else {
            *current_text = pending_text;
            QueryState::Changed
        }
    }

    fn reset(&self) {
        readline::reset();
        *self.text.borrow_mut() = String::new();
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
