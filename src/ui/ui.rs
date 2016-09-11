use std::cmp;

use ncurses::*;
use unicode_width::UnicodeWidthStr;

use ui::menu::Menu;

pub enum Direction {
    Left,
    Right
}

pub enum Event {
    SelectMenuItem(Direction),
    ScrollContents(i32),
    Other
}

pub struct Ui {
    menu: Menu,
    content: WINDOW,
    pub screen_lines: i32
}

impl Ui {
    pub fn new(menu_items: &Vec<String>) -> Ui {
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

        Ui {
            menu: Menu::new(LINES - 1, 0, menu_items),
            content: newpad(5000, COLS),
            screen_lines: 0
        }
    }

    pub fn render(&self) {
        self.menu.render(COLOR_PAIR(1), COLOR_PAIR(2));
    }

    pub fn select_left_menu_item(&self) {
        self.menu.select(REQ_LEFT_ITEM);
    }

    pub fn select_right_menu_item(&self) {
        self.menu.select(REQ_RIGHT_ITEM);
    }

    pub fn destroy(&self) {
        self.menu.destroy();
        endwin();
    }

    pub fn print<'a>(&mut self, data: (Box<Iterator<Item=&'a String> + 'a>, usize)) {
        let (lines, scroll_offset) = data;
        self.screen_lines = 0;

        for line in lines {
            self.screen_lines += self.calculate_line_height(line);
            wprintw(self.content, &format!("{}\n", line));
        }

        self.scroll(scroll_offset as i32);
    }

    fn calculate_line_height(&self, line: &str) -> i32 {
        let result = (line.width() as f32 / COLS as f32).ceil() as i32;
        cmp::max(1, result)
    }

    pub fn scroll(&self, reversed_offset: i32) {
        let offset =  self.screen_lines - LINES + 2 - reversed_offset;
        prefresh(self.content, offset, 0, 0, 0, LINES - 2, COLS);
    }

    pub fn refresh(&self) {
        wrefresh(self.content);
    }

    pub fn clear(&self) {
        wclear(self.content);
    }

    pub fn watch(&self) -> Event {
        match getch() {
            KEY_LEFT => Event::SelectMenuItem(Direction::Left),
            KEY_RIGHT => Event::SelectMenuItem(Direction::Right),
            KEY_UP => Event::ScrollContents(1),
            KEY_DOWN => Event::ScrollContents(-1),
            KEY_MOUSE => self.read_mouse_event(),
            _ => Event::Other
        }
    }

    fn read_mouse_event(&self) -> Event {
        let ref mut event = MEVENT {
            id: 0, x: 0, y: 0, z: 0, bstate: 0
        };
        if getmouse(event) == OK {
            if (event.bstate & BUTTON4_PRESSED as u64) != 0 {
                return Event::ScrollContents(1)
            } else if (event.bstate & BUTTON5_PRESSED as u64) != 0 {
                return Event::ScrollContents(-1)
            }
        }
        Event::Other
    }
}
