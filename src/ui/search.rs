use std::cell::RefCell;

use ncurses::*;

use ui::readline;

static OPTIONS_WIDTH: i32 = 29;
static WITH_MATCHES_COLOR_PAIR_ID: i16 = 1;
static NO_MATCHES_COLOR_PAIR_ID: i16 = 4;

pub struct Query {
    pub text: String,
    pub filter_mode: bool
}

pub struct Search {
    pub window: WINDOW,
    pub options: Options,
    pub input_field: InputField,
    pub matches_found: bool,
    panel: PANEL
}

impl Search {
    pub fn new(position_x: i32, position_y: i32) -> Search {
        let window = newwin(0, 0, position_x, position_y);

        Search {
            window: window,
            options: Options::new(window),
            input_field: InputField::new(window),
            panel: new_panel(window),
            matches_found: false
        }
    }

    pub fn render(&self) {
        let color_pair = COLOR_PAIR(self.color_pair_id());

        wbkgd(self.window, color_pair);
        self.input_field.render(color_pair);
        self.options.render(color_pair);
        wrefresh(self.window);
    }

    pub fn build_query(&self) -> Query {
        let query = Query {
            text: self.input_field.text.borrow().clone(),
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

    fn color_pair_id(&self) -> i16 {
        if !self.matches_found && self.input_field.text.borrow().len() > 0 {
            NO_MATCHES_COLOR_PAIR_ID
        } else  {
            WITH_MATCHES_COLOR_PAIR_ID
        }
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

    fn render(&self, color_pair: u64) {
        wclear(self.window);
        wbkgd(self.window, color_pair);
        wattron(self.window, color_pair);
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
    filter_mode: bool
}

impl Options {
    fn new(parent_window: WINDOW) -> Options {
        Options {
            window: derwin(parent_window, 1, OPTIONS_WIDTH, 0, COLS - OPTIONS_WIDTH),
            filter_mode: false
        }
    }

    fn render(&self, color_pair: u64) {
        wclear(self.window);
        readline::handle_redisplay();
        wbkgd(self.window, color_pair);

        self.print_navigation();
        self.print_filter(color_pair);
    }

    fn print_navigation(&self) {
        wprintw(self.window, " / ");
        self.make_shortcut('N');
        wprintw(self.window, "ext / ");
        self.make_shortcut('P');
        wprintw(self.window, "rev");
    }

    fn print_filter(&self, color_pair: u64) {
        wprintw(self.window, " / ");
        if self.filter_mode {
            wattron(self.window, COLOR_PAIR(2));
        }
        wprintw(self.window, "Filter ");
        self.make_shortcut('M');
        wprintw(self.window, "ode");
        if self.filter_mode {
            wattroff(self.window, color_pair);
        }
    }

    fn make_shortcut(&self, letter: char) {
        wattron(self.window, A_UNDERLINE());
        waddch(self.window, letter as u64);
        wattroff(self.window, A_UNDERLINE());
    }
}
