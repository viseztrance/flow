use ncurses::*;

pub struct Menu {
    pub window: WINDOW,
    panel: PANEL,
    object: MENU,
    items: Vec<ITEM>
}

impl Menu {
    pub fn new(position_x: i32, position_y: i32, item_names: &Vec<String>) -> Menu {
        let window = newwin(0, 0, position_x, position_y);

        let mut items = vec![];

        for name in item_names {
            items.push(new_item(name.as_str(), ""));
        }

        Menu {
            window: window,
            panel: new_panel(window),
            object: new_menu(&mut items),
            items: items
        }
    }

    pub fn select(&self, item: i32) {
        menu_driver(self.object, item);
        pos_menu_cursor(self.object);
        wrefresh(self.window);
    }

    pub fn render(&self, foreground: u64, background: u64) {
        keypad(self.window, true);

        set_menu_win(self.object, self.window);
        set_menu_sub(self.object, derwin(self.window, 0, 0, 0, 0));

        menu_opts_off(self.object, O_SHOWDESC);
        set_menu_mark(self.object, "");
        set_menu_fore(self.object, foreground);
        set_menu_back(self.object, background);
        set_menu_format(self.object, 1, self.items.len() as i32);
        post_menu(self.object);

        refresh();
        wbkgd(self.window, background);
        wrefresh(self.window);
    }

    pub fn show(&self) {
        show_panel(self.panel);
    }

    pub fn hide(&self) {
        hide_panel(self.panel);
    }

    pub fn destroy(&self) {
        unpost_menu(self.object);

        for &item in self.items.iter() {
            free_item(item);
        }

        free_menu(self.object);
    }
}