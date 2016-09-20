use std::collections::VecDeque;

use utils::ansi_decoder::{ComponentCollection, AnsiStr};

pub struct Line {
    pub content_without_ansi: String,
    pub components: Option<ComponentCollection>
}

impl Line {
    fn new(content: String) -> Line {
        let has_ansi = content.has_ansi_escape_sequence();

        let (content_without_ansi, components) = if has_ansi {
            (content.strip_ansi(), Some(content.to_components()))
        } else {
            (content, None)
        };

        Line {
            content_without_ansi: content_without_ansi,
            components: components
        }
    }
}

pub struct LineCollection {
    pub entries: VecDeque<Line>,
    capacity: usize
}

impl LineCollection {
    pub fn new(capacity: usize) -> LineCollection {
        LineCollection {
            entries: VecDeque::new(),
            capacity: capacity
        }
    }

    pub fn clear_excess(&mut self) {
        while self.entries.len() > self.capacity {
            self.entries.pop_front();
        }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    fn add(&mut self, item: String) {
        self.entries.push_back(Line::new(item));
    }
}

impl Extend<String> for LineCollection {
    fn extend<T: IntoIterator<Item=String>>(&mut self, iter: T) {
        for item in iter {
            self.add(item);
        }
    }
}
