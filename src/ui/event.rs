use ui::input::*;
use ui::navigation::State as NavigationState;

pub enum Direction {
    Left,
    Right
}

pub enum SearchAction {
    ReadInput(Vec<i32>),
    ToggleFilterMode,
    FindNextMatch,
    FindPreviousMatch,
}

pub enum Offset {
    Line(i32),
    Viewport(i32),
    Top,
    Bottom
}

pub enum Event {
    ScrollContents(Offset),
    SelectMenuItem(Direction),
    Navigation(NavigationState),
    Search(SearchAction),
    Resize,
    Quit,
    Other
}

pub struct EventBuilder {
    input: Input,
    key: i32
}

impl EventBuilder {
    pub fn new(input: Input, key: i32) -> EventBuilder {
        EventBuilder {
            input: input,
            key: key
        }
    }

    pub fn construct(&self, current_navigation_state: &NavigationState) -> Event {
        let mut result = self.create_global_event();

        if result.is_none() {
            result = match *current_navigation_state {
                NavigationState::Menu => self.create_menu_event(),
                NavigationState::Search => self.create_search_event()
            };
        }
        result.unwrap_or(Event::Other)
    }

    fn create_menu_event(&self) -> Option<Event> {
        match self.input {
            Input::Kb(Key::Left, None) => {
                Some(Event::SelectMenuItem(Direction::Left))
            },
            Input::Kb(Key::Right, None) => {
                Some(Event::SelectMenuItem(Direction::Right))
            },
            Input::Kb(Key::Char('/'), None) | Input::Kb(Key::Char('F'), Some(Modifier::Ctrl)) => {
                Some(Event::Navigation(NavigationState::Search))
            },
            Input::Kb(Key::Char('q'), None) => {
                Some(Event::Quit)
            },
            _ => None
        }
    }

    fn create_search_event(&self) -> Option<Event> {
        match self.input {
            Input::Kb(Key::Char('n'), Some(Modifier::Alt(_))) => {
                Some(Event::Search(SearchAction::FindNextMatch))
            },
            Input::Kb(Key::Char('p'), Some(Modifier::Alt(_))) => {
                Some(Event::Search(SearchAction::FindPreviousMatch))
            },
            Input::Kb(Key::Char('m'), Some(Modifier::Alt(_))) => {
                Some(Event::Search(SearchAction::ToggleFilterMode))
            },
            Input::Kb(Key::Escape, None) => {
                Some(Event::Navigation(NavigationState::Menu))
            },
            _ => self.create_input_event()
        }
    }

    fn create_global_event(&self) -> Option<Event> {
        match self.input {
            Input::Kb(Key::Up, None) => {
                Some(Event::ScrollContents(Offset::Line(1)))
            },
            Input::Kb(Key::Down, None) => {
                Some(Event::ScrollContents(Offset::Line(-1)))
            },
            Input::Kb(Key::PageUp, None) => {
                Some(Event::ScrollContents(Offset::Viewport(1)))
            },
            Input::Kb(Key::PageDown, None) => {
                Some(Event::ScrollContents(Offset::Viewport(-1)))
            },
            Input::Kb(Key::Home, None) => {
                Some(Event::ScrollContents(Offset::Top))
            },
            Input::Kb(Key::End, None) => {
                Some(Event::ScrollContents(Offset::Bottom))
            },
            Input::Resize => Some(Event::Resize),
            _ => None
        }
    }

    fn create_input_event(&self) -> Option<Event> {
        match self.input {
            Input::Kb(Key::Left, None) => {
                Some(Event::Search(SearchAction::ReadInput(KEY_LEFT_SEQ.to_vec())))
            },
            Input::Kb(Key::Right, None) => {
                Some(Event::Search(SearchAction::ReadInput(KEY_RIGHT_SEQ.to_vec())))
            },
            Input::Kb(Key::Home, None) => {
                Some(Event::Search(SearchAction::ReadInput(KEY_HOME_SEQ.to_vec())))
            },
            Input::Kb(Key::End, None) => {
                Some(Event::Search(SearchAction::ReadInput(KEY_END_SEQ.to_vec())))
            },
            Input::Kb(Key::Delete, None) => {
                let mut keys = KEY_RIGHT_SEQ.to_vec();
                keys.extend(KEY_BACKSPACE_SEQ.to_vec());
                Some(Event::Search(SearchAction::ReadInput(keys)))
            },
            Input::Kb(Key::Backspace, None) => {
                Some(Event::Search(SearchAction::ReadInput(KEY_BACKSPACE_SEQ.to_vec())))
            },
            Input::Kb(_, ref modifier) => {
                let mut keys = vec![self.key];
                match *modifier {
                    Some(Modifier::Alt(value)) => { keys.push(value) },
                    _ => {}
                };
                Some(Event::Search(SearchAction::ReadInput(keys)))
            },
            _ => None
        }
    }
}
