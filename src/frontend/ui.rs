use ncurses::*;

use core::line::Line;
use frontend::readline;
use frontend::input::read_key;
use frontend::event::{EventBuilder, Event};
use frontend::navigation::Navigation;
use frontend::content::Content;
use frontend::printer::Print;
use frontend::color;

static MAX_SCROLLING_LINES: i32 = 10_000;

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
        readline::render("Search:", self.navigation.search.input_field.window);

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
        EventBuilder::new(input, key).construct(&self.navigation.state)
    }
}
