use ncurses::*;
use ui::color::COLOR_DEFAULT;
use std::cell::RefCell;

pub struct Content {
    pub window: WINDOW,
    pub state: RefCell<State>
}

impl Content {
    pub fn new(width: i32, height: i32) -> Content {
        Content {
            window: newpad(width, height),
            state: RefCell::new(State::default())
        }
    }

    pub fn render(&self) {
        scrollok(self.window, true);
    }

    pub fn refresh(&self) {
        wrefresh(self.window);
    }

    pub fn clear(&self) {
        wclear(self.window);
    }

    pub fn resize(&self, width: i32, height: i32) {
        wresize(self.window, width, height);
    }

    pub fn height(&self) -> i32 {
        let mut current_x: i32 = 0;
        let mut current_y: i32 = 0;
        getyx(self.window, &mut current_y, &mut current_x);

        current_y
    }
}

pub struct State {
    pub attributes: Vec<(usize, fn() -> u64)>,
    pub foreground: i16,
    pub background: i16
}

impl State {
    pub fn default() -> State {
        State {
            attributes: vec![],
            foreground: COLOR_DEFAULT,
            background: COLOR_DEFAULT
        }
    }

    pub fn remove_attribute(&mut self, attr_id: usize) {
        match self.attributes.iter().position(|cur| cur.0 == attr_id) {
            Some(index) => {
                self.attributes.remove(index);
            },
            _ => {}
        }
    }
}
