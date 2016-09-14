use std::sync::Mutex;

use ncurses::*;

use flow::line::Line;
use ui::ansi_converter::{CursesComponent, CursesStyle};
use ui::color::{ColorPair, COLOR_DEFAULT};

lazy_static! {
    static ref ACTIVE_VALUES: Mutex<(Vec<fn() -> u64>, i16, i16)> = Mutex::new((vec![], COLOR_DEFAULT, COLOR_DEFAULT));
}

pub trait Print {
    fn print<'a>(&'a self, window: WINDOW);
}

impl Print for Line {
    fn print(&self, window: WINDOW) {
        match self.components {
            Some(ref value) => {
                for component in value {
                    component.print(window);
                }
                waddch(window, '\n' as u64);
            },
            None => {
                wprintw(window, &format!("{}\n", self.content_without_ansi));
            }
        };
    }
}

impl Print for CursesComponent {
    fn print(&self, window: WINDOW) {
        match self {
            &CursesComponent::Style(value) => {
                value.print(window);
            },
            &CursesComponent::Content(ref value) => {
                wprintw(window, &value);
            }
        };
    }
}

impl Print for CursesStyle {
    fn print(&self, window: WINDOW) {
        let mut active_values = ACTIVE_VALUES.lock().unwrap();

        match self {
            &CursesStyle::Attribute(prop, active) => {
                if active {
                    active_values.0.push(prop);
                    wattron(window, prop() as i32);
                } else {
                    wattroff(window, prop() as i32);
                }
            },
            &CursesStyle::Color(foreground, background) => {
                let color = ColorPair::from_options(
                    foreground,
                    background,
                    active_values.1,
                    active_values.2
                );
                wattron(window, color.to_attr());

                active_values.1 = color.foreground;
                active_values.2 = color.background;
            },
            &CursesStyle::Reset => {
                for prop in active_values.0.drain(..) {
                    wattroff(window, prop() as i32);
                }

                wattron(window, ColorPair::default().to_attr());
            }
        }
    }
}
