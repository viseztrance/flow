use regex::Regex;

#[derive(RustcDecodable)]
pub struct Filter {
    pub name: String,
    pub rule: String
}

impl Filter {
    pub fn parse<'a, I, F>(&self, lines: I, callback: F) where I : IntoIterator<Item = &'a String>, F : Fn(&str) {
        let regex = Regex::new(&self.rule).unwrap();

        for line in lines {
            if regex.is_match(line) {
                callback(line);
            }
        }
    }
}

pub struct FilterCollection {
    items: Vec<Filter>,
    index: usize
}

impl FilterCollection {
    pub fn new(items: Vec<Filter>) -> FilterCollection {
        FilterCollection {
            items: items,
            index: 0
        }
    }

    pub fn selected_item(&self) -> &Filter {
        self.items.get(self.index).unwrap()
    }

    pub fn select_left_item(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        }
    }

    pub fn select_right_item(&mut self) {
        if self.index + 1 < self.items.len() {
            self.index += 1;
        }
    }
}
