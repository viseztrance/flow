use ncurses::*;

// A negative value is interpreted as the default (original) color.
// For color pair generation, when unsigned it must also be different than the 8 colors already defined.
pub static COLOR_DEFAULT: i16 = -9;

pub struct ColorPair {
    pub foreground: i16,
    pub background: i16
}

impl ColorPair {
    pub fn new(foreground: i16, background: i16) -> ColorPair {
        ColorPair {
            foreground: foreground,
            background: background
        }
    }

    pub fn from_options(foreground: Option<i16>,
                        background: Option<i16>,
                        fallback_foreground: i16,
                        fallback_background: i16) -> ColorPair {
        ColorPair::new(
            foreground.unwrap_or(fallback_foreground),
            background.unwrap_or(fallback_background)
        )
    }

    pub fn default() -> ColorPair {
        ColorPair::new(COLOR_DEFAULT, COLOR_DEFAULT)
    }

    fn calculate_id(&self) -> i16 {
        100 + self.foreground.abs() * 10 + self.background.abs()
    }

    fn init(&self) {
        init_pair(self.calculate_id(), self.foreground, self.background);
    }

    pub fn to_attr(&self) -> u64 {
        COLOR_PAIR(self.calculate_id())
    }
}

pub fn generate_pairs() {
    let colors = [
        COLOR_BLACK,
        COLOR_RED,
        COLOR_GREEN,
        COLOR_YELLOW,
        COLOR_BLUE,
        COLOR_MAGENTA,
        COLOR_CYAN,
        COLOR_WHITE,
        COLOR_DEFAULT
    ];

    for foreground in colors.iter() {
        for background in colors.iter() {
            ColorPair::new(*foreground, *background).init();
        }
    }
}
