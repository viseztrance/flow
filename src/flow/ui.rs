use ncurses::*;

pub enum Key {
    Left,
    Right,
    Unknown
}

pub struct Ui {
    menu: Menu,
    content: WINDOW
}

impl Ui {
    pub fn new(menu_items: &Vec<String>) -> Ui {
        initscr();
        start_color();
        cbreak();
        noecho();
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        halfdelay(1);
        keypad(stdscr, true);
        init_pair(1, COLOR_WHITE, COLOR_BLUE);
        init_pair(2, COLOR_WHITE, COLOR_GREEN);
        init_pair(3, COLOR_WHITE, COLOR_YELLOW);

        Ui {
            menu: Menu::new(LINES - 1, 0, menu_items),
            content: newwin(LINES - 1, COLS, 0, 0)
        }
    }

    pub fn render(&self) {
        self.menu.render(COLOR_PAIR(1), COLOR_PAIR(2));

        scrollok(self.content, true);
        wbkgd(self.content, COLOR_PAIR(3));
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

    pub fn print<'a, I>(&self, lines: I) where I: IntoIterator<Item = &'a String> {
        for line in lines {
            wprintw(self.content, &format!("{}\n", line));
        }

        wrefresh(self.content);
    }

    pub fn clear(&self) {
        wclear(self.content);
    }

    pub fn read_input(&self) -> Key {
        match getch() {
            KEY_LEFT => Key::Left,
            KEY_RIGHT => Key::Right,
            _ => Key::Unknown
        }
    }
}

struct Menu {
    object: MENU,
    window: WINDOW,
    items: Vec<ITEM>
}

impl Menu {
    fn new(position_x: i32, position_y: i32, values: &Vec<String>) -> Menu {
        let mut items = vec![];

        for value in values {
            items.push(new_item(value.as_str(), ""));
        }

        let object = new_menu(&mut items);

        Menu {
            object: object,
            items: items,
            window: newwin(0, 0, position_x, position_y)
        }
    }

    fn render(&self, foreground: u64, background: u64) {
        keypad(self.window, true);

        set_menu_win(self.object, self.window);
        set_menu_sub(self.object, derwin(self.window, 0, 0, 0, 0));

        menu_opts_off(self.object, O_SHOWDESC);
        set_menu_mark(self.object, "");
        set_menu_fore(self.object, foreground);
        set_menu_back(self.object, background);
        set_menu_format(self.object, 1, self.items.len() as i32);

        refresh();

        wbkgd(self.window, background);

        post_menu(self.object);
        wrefresh(self.window);
    }

    fn select(&self, item: i32) {
        menu_driver(self.object, item);
        pos_menu_cursor(self.object);
        wrefresh(self.window);
    }

    fn destroy(&self) {
        unpost_menu(self.object);

        for &item in self.items.iter() {
            free_item(item);
        }

        free_menu(self.object);
    }
}
