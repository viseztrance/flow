use ncurses::*;

use frontend::menu::Menu;
use frontend::search::Search;

#[derive(PartialEq)]
pub enum State {
    Menu,
    Search
}

pub struct Navigation {
    pub menu: Menu,
    pub search: Search,
    pub state: State
}

impl Navigation {
    pub fn new(position_x: i32, position_y: i32, menu_item_names: &Vec<String>) -> Navigation {
        Navigation {
            menu: Menu::new(position_x, position_y, menu_item_names),
            search: Search::new(position_x, position_y),
            state: State::Menu
        }
    }

    pub fn render(&self) {
        self.search.render();
        self.menu.render();
        self.handle_visibility();
    }

    pub fn destroy(&self) {
        self.menu.destroy();
    }

    pub fn change_state(&mut self, new_state: State) {
        if self.state == new_state {
            return;
        }
        self.state = new_state;
        self.handle_visibility();
    }

    pub fn resize(&self, position_x: i32, position_y: i32) {
        mvwin(self.search.window, position_y, position_x);
        mvwin(self.search.input.window, position_y, position_x + 1);
        wrefresh(self.search.input.window);
        mvwin(self.menu.window, position_y, position_x);

        self.render();
    }

    fn handle_visibility(&self) {
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
}
