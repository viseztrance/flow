use ncurses::*;

use core::line::Line;
use utils::ansi_decoder::{Component, Style};
use frontend::color::ColorPair;
use frontend::content::Content;

pub trait Print {
    fn print(&self, content: &Content);
}

impl Print for Line {
    fn print(&self, content: &Content) {
        match self.components {
            Some(ref value) => {
                for component in &value.items {
                    component.print(content);
                }
                waddch(content.window, '\n' as u64);
            },
            None => {
                wprintw(content.window, &format!("{}\n", self.content_without_ansi));
            }
        };
    }
}

impl Print for Component {
    fn print(&self, content: &Content) {
        match *self {
            Component::Style(value) => {
                value.print(content);
            },
            Component::Content(ref value) => {
                wprintw(content.window, value);
            }
        };
    }
}

impl Print for Style {
    fn print(&self, content: &Content) {
        let mut state = content.state.borrow_mut();

        match *self {
            Style::Attribute(id, prop, active) => {
                if active {
                    state.attributes.push((id, prop));
                    wattron(content.window, prop());
                } else {
                    wattroff(content.window, prop());
                    state.remove_attribute(id);
                }
            },
            Style::Color(foreground, background) => {
                let color = ColorPair::from_options(
                    foreground,
                    background,
                    state.foreground,
                    state.background
                );
                wattron(content.window, color.to_attr());

                state.foreground = color.foreground;
                state.background = color.background;
            },
            Style::Reset => {
                for (_, prop) in state.attributes.drain(..) {
                    wattroff(content.window, prop());
                }

                wattron(content.window, ColorPair::default().to_attr());
            }
        }
    }
}
