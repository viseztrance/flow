use ncurses::*;

use flow::line::Line;
use ui::key::{Key, read_key};
use ui::navigation::navigation::{Navigation, State as NavigationState};
use ui::content::Content;
use ui::printer::Print;
use ui::color;

static MAX_SCROLLING_LINES: i32 = 10_000;

pub enum Direction {
    Left,
    Right
}

pub enum Event {
    ScrollContents(i32),
    SelectMenuItem(Direction),
    Navigation(NavigationState),
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
        setlocale(LcCategory::all, ""); // Must be set *before* init

        initscr();
        start_color();
        use_default_colors();
        cbreak();
        noecho();
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        halfdelay(1);
        mouseinterval(0);
        keypad(stdscr, true);

        init_pair(1, COLOR_WHITE, COLOR_BLUE);
        init_pair(2, COLOR_WHITE, COLOR_GREEN);
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
        self.navigation.render(COLOR_PAIR(1), COLOR_PAIR(2));
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
        match read_key() {
            Key::Left   => Event::SelectMenuItem(Direction::Left),
            Key::Right  => Event::SelectMenuItem(Direction::Right),
            Key::Up | Key::MouseWheelUp => Event::ScrollContents(1),
            Key::Down | Key::MouseWheelDown => Event::ScrollContents(-1),
            Key::Resize => Event::Resize,
            Key::Char('/') => Event::Navigation(NavigationState::Search),
            Key::Char('m') => Event::Navigation(NavigationState::Menu),
            _ => Event::Other
        }
    }
}
