use ui::input::*;
use ui::navigation::navigation::State as NavigationState;

pub enum Direction {
    Left,
    Right
}

pub enum SearchAction {
    ReadInput(Vec<i32>),
    ToggleHighlightMode,
    ToggleFilterMode,
    FindNextMatch,
    FindPreviousMatch,
}

pub enum Event {
    ScrollContents(i32),
    SelectMenuItem(Direction),
    Navigation(NavigationState),
    Search(SearchAction),
    Resize,
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
            Input::Kb(Key::Char('/'), None) => {
                Some(Event::Navigation(NavigationState::Search))
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
            Input::Kb(Key::Char('a'), Some(Modifier::Alt(_))) => {
                Some(Event::Search(SearchAction::ToggleHighlightMode))
            },
            Input::Kb(Key::Char('f'), Some(Modifier::Alt(_))) => {
                Some(Event::Search(SearchAction::ToggleFilterMode))
            },
            Input::Kb(Key::Escape, None) => {
                Some(Event::Navigation(NavigationState::Menu))
            },
            Input::Kb(Key::Tab, None) => None,
            _ => self.create_input_event()
        }
    }

    fn create_global_event(&self) -> Option<Event> {
        match self.input {
            Input::Kb(Key::Up, None) => {
                Some(Event::ScrollContents(1))
            },
            Input::Kb(Key::Down, None) => {
                Some(Event::ScrollContents(-1))
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
