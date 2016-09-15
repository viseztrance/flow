use ncurses::*;

use ui::navigation::menu::Menu;
use ui::navigation::search::Search;

#[derive(PartialEq)]
pub enum State {
    Menu,
    Search
}

pub struct Navigation {
    pub menu: Menu,
    pub search: Search,
    state: State
}

impl Navigation {
    pub fn new(position_x: i32, position_y: i32, menu_item_names: &Vec<String>) -> Navigation {
        Navigation {
            menu: Menu::new(position_x, position_y, menu_item_names),
            search: Search::new(position_x, position_y),
            state: State::Menu
        }
    }

    pub fn render(&self, foreground: u64, background: u64) {
        self.search.render(foreground, background);
        self.search.hide();
        self.menu.render(foreground, background);
    }

    pub fn destroy(&self) {
        self.menu.destroy();
    }

    pub fn change_state(&mut self, new_state: State) {
        if self.state == new_state {
            return;
        }
        self.state = new_state;

        match self.state {
            State::Menu => {
                self.search.hide();
                self.menu.show();
            },
            State::Search => {
                self.menu.hide();
                self.search.show();
            }
        }

        update_panels();
        doupdate();
    }

    pub fn resize(&self, position_x: i32, position_y: i32) {
        mvwin(self.menu.window, position_y, position_x);
        refresh();
        wrefresh(self.menu.window);
    }
}
