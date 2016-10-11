use unicode_width::UnicodeWidthStr;

use ncurses::*;

use core::line::Line;
use ui::readline;
use ui::color;
use ui::input::read_key;
use ui::event::{EventBuilder, Event};
use ui::navigation::Navigation;
use ui::plane::Plane;
use ui::content::Content;
use ui::printer::Print;
use ui::search::Query;

static MAX_SCROLLING_LINES: i32 = 15_000;

pub struct Frame {
    pub plane: Plane,
    pub navigation: Navigation,
    pub content: Content
}

impl Frame {
    pub fn new(menu_item_names: &Vec<String>) -> Frame {
        ::std::env::set_var("ESCDELAY", "25");
        setlocale(LcCategory::all, ""); // Must be set *before* ncurses init

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
        init_pair(4, COLOR_WHITE, COLOR_MAGENTA);
        color::generate_pairs();
        let plane = Plane::new();

        Frame {
            navigation: Navigation::new(plane.height - 1, 0, menu_item_names),
            content: Content::new(MAX_SCROLLING_LINES, plane.width),
            plane: plane
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
        let (lines, scroll_offset) = data;

        LinesPrinter::new(self, lines).draw(query_opt);
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
}

struct LinesPrinter<'a> {
    lines: Option<Box<Iterator<Item=&'a Line> + 'a>>,
    frame: &'a mut Frame,
    height: i32
}

impl<'a> LinesPrinter<'a> {
    pub fn new(frame: &'a mut Frame, lines: Box<Iterator<Item=&'a Line> + 'a>) -> LinesPrinter<'a> {
        LinesPrinter {
            frame: frame,
            lines: Some(lines),
            height: 0
        }
    }

    pub fn draw(&mut self, query_opt: Option<Query>) {
        self.frame.plane.lines.clear();
        self.frame.navigation.search.matches_found = false;
        self.height = 0;

        if let Some(ref query) = query_opt {
            for line in self.lines.take().unwrap() {
                let is_match = line.content_without_ansi.contains(&query.text);

                if !query.filter_mode || (query.filter_mode && is_match) {
                    let height = self.frame.content.calculate_height_change(||{
                        line.print(&self.frame.content);
                    });

                    if is_match {
                        self.frame.navigation.search.matches_found = true;
                        self.highlight(line, query, height);
                    }

                    self.height += height;
                    self.frame.plane.lines.push(height);
                }
            }
        } else {
            for line in self.lines.take().unwrap() {
                let height = self.frame.content.calculate_height_change(||{
                    line.print(&self.frame.content);
                });
                self.height += height;
                self.frame.plane.lines.push(height);
            }
        }
    }

    fn highlight(&self, line: &Line, query: &Query, height: i32) {
        let matches: Vec<_> = line.content_without_ansi.match_indices(&query.text).collect();
        for (i, value) in matches {
            let mut offset_x = i as i32;
            let mut offset_y  = self.height;
            if offset_x > self.frame.plane.width {
                offset_x = line.content_without_ansi.split_at(i).0.width() as i32;
                offset_y = (offset_x / self.frame.plane.width) + offset_y;
                offset_x = offset_x % self.frame.plane.width;
            }
            wattron(self.frame.content.window, A_STANDOUT());
            mvwprintw(self.frame.content.window, offset_y, offset_x, value);
            wattroff(self.frame.content.window, A_STANDOUT());
        }
        wmove(self.frame.content.window, self.height + height, 0);
    }
}
