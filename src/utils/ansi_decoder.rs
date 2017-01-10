/**
 * Flow - Realtime log analyzer
 * Copyright (C) 2016 Daniel Mircea
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use std::collections::HashMap;
use regex::Regex;
use ncurses::*;

use ui::color::COLOR_DEFAULT;

lazy_static! {
    static ref ANSI_TO_NCURSES_MAPPING: HashMap<&'static str, Style> = {
        let mut codes = HashMap::new();

        codes.insert("[0m", Style::Reset);

        codes.insert("[1m", Style::Attribute(1, A_BOLD, true));
        codes.insert("[3m", Style::Attribute(2, A_STANDOUT, true)); // Italic
        codes.insert("[4m", Style::Attribute(3, A_UNDERLINE, true));
        codes.insert("[7m", Style::Attribute(4, A_REVERSE, true));
        codes.insert("[9m", Style::Attribute(5, A_DIM, true)); // Strikethrough

        codes.insert("[22m", Style::Attribute(1, A_BOLD, false));
        codes.insert("[23m", Style::Attribute(2, A_STANDOUT, false)); // Italic
        codes.insert("[24m", Style::Attribute(3, A_UNDERLINE, false));
        codes.insert("[27m", Style::Attribute(4, A_REVERSE, false));
        codes.insert("[29m", Style::Attribute(5, A_DIM, false)); // Strikethrough

        codes.insert("[30m", Style::Color(Some(COLOR_BLACK), None));
        codes.insert("[31m", Style::Color(Some(COLOR_RED), None));
        codes.insert("[32m", Style::Color(Some(COLOR_GREEN), None));
        codes.insert("[33m", Style::Color(Some(COLOR_YELLOW), None));
        codes.insert("[34m", Style::Color(Some(COLOR_BLUE), None));
        codes.insert("[35m", Style::Color(Some(COLOR_MAGENTA), None));
        codes.insert("[36m", Style::Color(Some(COLOR_CYAN), None));
        codes.insert("[37m", Style::Color(Some(COLOR_WHITE), None));
        codes.insert("[39m", Style::Color(Some(COLOR_DEFAULT), None));

        codes.insert("[40m", Style::Color(None, Some(COLOR_BLACK)));
        codes.insert("[41m", Style::Color(None, Some(COLOR_RED)));
        codes.insert("[42m", Style::Color(None, Some(COLOR_GREEN)));
        codes.insert("[43m", Style::Color(None, Some(COLOR_YELLOW)));
        codes.insert("[44m", Style::Color(None, Some(COLOR_BLUE)));
        codes.insert("[45m", Style::Color(None, Some(COLOR_MAGENTA)));
        codes.insert("[46m", Style::Color(None, Some(COLOR_CYAN)));
        codes.insert("[47m", Style::Color(None, Some(COLOR_WHITE)));
        codes.insert("[49m", Style::Color(None, Some(COLOR_DEFAULT)));

        codes
    };
}

#[derive(Clone)]
pub enum Component {
    Style(&'static Style),
    Content(String),
}

#[derive(Clone)]
pub struct ComponentCollection {
    pub items: Vec<Component>,
}

impl ComponentCollection {
    fn from_string(value: &str) -> ComponentCollection {
        let mut components = ComponentCollection::new();

        lazy_static! {
            static ref BREAK_ANSI_MATCHER: Regex = Regex::new(r"(\x1b\[\d+m)|([^\x1b]*)").unwrap();
        }

        for capture in BREAK_ANSI_MATCHER.captures_iter(value) {
            if capture.at(1).is_some() {
                if let Some(style) = ANSI_TO_NCURSES_MAPPING.get(capture.at(1).unwrap()) {
                    components.push(Component::Style(style))
                }
            }
            if capture.at(2).is_some() {
                let content = capture.at(2).unwrap().to_string();
                components.push(Component::Content(content));
            }
        }

        components
    }

    fn new() -> ComponentCollection {
        ComponentCollection { items: Vec::new() }
    }

    fn push(&mut self, item: Component) {
        self.items.push(item);
    }
}

pub enum Style {
    Attribute(usize, fn() -> attr_t, bool),
    Color(Option<i16>, Option<i16>),
    Reset,
}

pub trait AnsiStr {
    fn has_ansi_escape_sequence(&self) -> bool;

    fn strip_ansi(&self) -> String;

    fn to_components(&self) -> ComponentCollection;
}

impl AnsiStr for str {
    fn has_ansi_escape_sequence(&self) -> bool {
        self.contains('')
    }

    fn strip_ansi(&self) -> String {
        lazy_static! {
            static ref STRIP_ANSI_MATCHER: Regex = Regex::new(r"(\x1b\[\d+m)").unwrap();
        }
        STRIP_ANSI_MATCHER.replace_all(self, "")
    }

    fn to_components(&self) -> ComponentCollection {
        ComponentCollection::from_string(self)
    }
}
