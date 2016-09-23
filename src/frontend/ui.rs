use ncurses::*;

use core::line::Line;
use frontend::readline;
use frontend::color;
use frontend::input::read_key;
use frontend::event::{EventBuilder, Event};
use frontend::navigation::Navigation;
use frontend::plane::Plane;
use frontend::content::Content;
use frontend::printer::Print;
use frontend::search::Query;

static MAX_SCROLLING_LINES: i32 = 15_000;

pub struct Ui {
    pub plane: Plane,
    pub navigation: Navigation,
    pub content: Content
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
        let plane = Plane::new();

        Ui {
            navigation: Navigation::new(plane.height - 1, 0, menu_item_names),
            content: Content::new(MAX_SCROLLING_LINES, plane.width),
            plane: plane,
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
        self.plane.resize();

        self.content.resize(MAX_SCROLLING_LINES, self.plane.width);
        self.navigation.resize(0, self.plane.height - 1);
    }

    pub fn print<'a>(&mut self, data: (Box<Iterator<Item=&'a Line> + 'a>, usize), query_opt: Option<Query>) {
        self.plane.lines.clear();

        let (lines, scroll_offset) = data;
        let mut current_height = 0;


        for line in lines {
            let current_line_height = self.content.calculate_height_change(||{
                line.print(&self.content);
            });

            if let Some(ref query) = query_opt {
                self.highlight_matches(line, query, current_height, current_line_height);
            }
            current_height += current_line_height;
            self.plane.lines.push(current_line_height);
        }

        self.scroll(scroll_offset as i32);
    }

    pub fn scroll(&self, reversed_offset: i32) {
        let offset =  self.plane.virtual_height() - self.plane.height + 1 - reversed_offset;
        prefresh(self.content.window, offset, 0, 0, 0, self.plane.height - 2, self.plane.width);
    }

    pub fn watch(&self) -> Event {
        let (input, key) = read_key();
        EventBuilder::new(input, key).construct(&self.navigation.state)
    }

    pub fn highlight_matches(&self, line: &Line, query: &Query, total_height: i32, line_height: i32) {
        let matches: Vec<_> = line.content_without_ansi.match_indices(&query.text).collect();
        for (i, value) in matches {
            wattron(self.content.window, A_STANDOUT());
            mvwprintw(self.content.window, total_height, i as i32, value);
            wattroff(self.content.window, A_STANDOUT());
        }
        mvwprintw(self.content.window, total_height + line_height, 0, "");
    }
}
