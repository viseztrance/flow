use ncurses::*;

use flow::line::Line;
use ui::readline;
use ui::input::*;
use ui::navigation::navigation::{Navigation, State as NavigationState};
use ui::content::Content;
use ui::printer::Print;
use ui::color;

static MAX_SCROLLING_LINES: i32 = 10_000;

pub enum Direction {
    Left,
    Right
}

pub enum SearchAction {
    ReadInput(Vec<i32>),
    ToggleHighlightMode,
    ToggleFilterMode,
    FindNextMatch,
    FindPreviousMatch,
}

pub enum Event {
    ScrollContents(i32),
    SelectMenuItem(Direction),
    Navigation(NavigationState),
    Search(SearchAction),
    Resize,
    Other
}

pub struct Ui {
    pub screen_lines: i32,
    pub navigation: Navigation,
    pub content: Content,
    height: i32,
    width: i32
}

impl Ui {
    pub fn new(menu_item_names: &Vec<String>) -> Ui {
        ::std::env::set_var("ESCDELAY", "25");
        setlocale(LcCategory::all, ""); // Must be set *before* init

        readline::init();

        initscr();
        start_color();
        use_default_colors();
        cbreak();
        noecho();
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        halfdelay(1);
        keypad(stdscr, true);

        init_pair(1, COLOR_WHITE, COLOR_BLUE);
        init_pair(2, COLOR_BLACK, COLOR_YELLOW);
        init_pair(3, COLOR_YELLOW, COLOR_BLUE);
        color::generate_pairs();

        Ui {
            navigation: Navigation::new(LINES - 1, 0, menu_item_names),
            content: Content::new(MAX_SCROLLING_LINES, COLS),
            screen_lines: 0,
            height: LINES,
            width: COLS
        }
    }

    pub fn render(&self) {
        readline::render("Search:", self.navigation.search.input.window);

        self.navigation.render();
        self.content.render();
    }

    pub fn select_left_menu_item(&self) {
        self.navigation.menu.select(REQ_LEFT_ITEM);
    }

    pub fn select_right_menu_item(&self) {
        self.navigation.menu.select(REQ_RIGHT_ITEM);
    }

    pub fn destroy(&self) {
        self.navigation.destroy();
        endwin();
        readline::terminate();
    }

    pub fn resize(&mut self) {
        getmaxyx(stdscr, &mut self.height, &mut self.width);

        self.content.resize(MAX_SCROLLING_LINES, self.width);
        self.navigation.resize(0, self.height - 1);
    }

    pub fn print<'a>(&mut self, data: (Box<Iterator<Item=&'a Line> + 'a>, usize)) {
        let (lines, scroll_offset) = data;

        for line in lines {
            line.print(&self.content);
        }

        self.screen_lines = self.content.height();
        self.scroll(scroll_offset as i32);
    }

    pub fn scroll(&self, reversed_offset: i32) {
        let offset =  self.screen_lines - self.height + 1 - reversed_offset;
        prefresh(self.content.window, offset, 0, 0, 0, self.height - 2, self.width);
    }

    pub fn watch(&self) -> Event {
        let (input, key) = read_key();

        let mut result = self.get_global_event(&input);

        if result.is_none() {
            result = match self.navigation.state {
                NavigationState::Menu => self.get_menu_event(&input),
                NavigationState::Search => self.get_search_event(&input, key),
            };
        }
        result.unwrap_or(Event::Other)
    }

    fn get_menu_event(&self, input: &Input) -> Option<Event> {
        match *input {
            Input::Kb(Key::Left, None) => {
                Some(Event::SelectMenuItem(Direction::Left))
            },
            Input::Kb(Key::Right, None) => {
                Some(Event::SelectMenuItem(Direction::Right))
            },
            Input::Kb(Key::Char('/'), None) => {
                Some(Event::Navigation(NavigationState::Search))
            },
            _ => None
        }
    }

    fn get_search_event(&self, input: &Input, key: i32) -> Option<Event> {
        match *input {
            Input::Kb(Key::Char('n'), Some(Modifier::Alt(_))) => {
                Some(Event::Search(SearchAction::FindNextMatch))
            },
            Input::Kb(Key::Char('p'), Some(Modifier::Alt(_))) => {
                Some(Event::Search(SearchAction::FindPreviousMatch))
            },
            Input::Kb(Key::Char('a'), Some(Modifier::Alt(_))) => {
                Some(Event::Search(SearchAction::ToggleHighlightMode))
            },
            Input::Kb(Key::Char('f'), Some(Modifier::Alt(_))) => {
                Some(Event::Search(SearchAction::ToggleFilterMode))
            },
            Input::Kb(Key::Escape, None) => {
                Some(Event::Navigation(NavigationState::Menu))
            },
            _ => self.get_input_event(input, key)
        }
    }

    fn get_global_event(&self, input: &Input) -> Option<Event> {
        match *input {
            Input::Kb(Key::Up, None) => {
                Some(Event::ScrollContents(1))
            },
            Input::Kb(Key::Down, None) => {
                Some(Event::ScrollContents(-1))
            },
            Input::Resize => Some(Event::Resize),
            _ => None
        }
    }

    fn get_input_event(&self, input: &Input, key: i32) -> Option<Event> {
        match *input {
            Input::Kb(Key::Left, None) => {
                Some(Event::Search(SearchAction::ReadInput(KEY_LEFT_SEQ.to_vec())))
            },
            Input::Kb(Key::Right, None) => {
                Some(Event::Search(SearchAction::ReadInput(KEY_RIGHT_SEQ.to_vec())))
            },
            Input::Kb(Key::Home, None) => {
                Some(Event::Search(SearchAction::ReadInput(KEY_HOME_SEQ.to_vec())))
            },
            Input::Kb(Key::End, None) => {
                Some(Event::Search(SearchAction::ReadInput(KEY_END_SEQ.to_vec())))
            },
            Input::Kb(Key::Delete, None) => {
                Some(Event::Search(SearchAction::ReadInput(KEY_DELETE_SEQ.to_vec())))
            },
            Input::Kb(Key::Backspace, None) => {
                Some(Event::Search(SearchAction::ReadInput(KEY_BACKSPACE_SEQ.to_vec())))
            },
            Input::Kb(_, ref modifier) => {
                let mut keys = vec![key];
                match *modifier {
                    Some(Modifier::Alt(value)) => { keys.push(value) },
                    _ => {}
                };
                Some(Event::Search(SearchAction::ReadInput(keys)))
            },
            _ => None
        }
    }
}
