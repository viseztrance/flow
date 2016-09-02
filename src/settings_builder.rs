use std::process;
use std::path::Path;
use getopts::{Options, Matches};

static DEFAULT_LAST_LINES_SHOWN: usize = 10;

pub struct Settings {
    pub path_to_target_file: String,
    pub path_to_config_file: String,
    pub line_count: usize
}

pub struct SettingsBuilder {
    program_name: String,
    options: Options,
    matches: Matches
}

impl SettingsBuilder {
    pub fn new(args: Vec<String>) -> SettingsBuilder {
        let opts = build_opts();
        let matches = opts.parse(&args[1..]).unwrap();

        SettingsBuilder {
            program_name: args.get(0).unwrap().clone(),
            options: opts,
            matches: matches
        }
    }

    pub fn make(&self) -> Settings {
        if self.matches.opt_present("h") {
            self.print_usage();
            process::exit(0);
        }

        Settings {
            path_to_target_file: self.get_target(),
            path_to_config_file: self.get_config(),
            line_count: self.get_lines()
        }
    }

    fn print_usage(&self) {
        let message = format!("Usage: {} FILE [options]", self.program_name);
        print!("{}", self.options.usage(&message));
    }

    fn get_target(&self) -> String {
        match self.matches.free.get(0) {
            Some(value) => {
                self.assert_file_exists(value);
                value.to_string()
            },
            None => {
                self.print_usage();
                process::exit(1);
            }
        }
    }

    fn get_config(&self) -> String {
        match self.matches.opt_str("c") {
            Some(value) => value.to_string(),
            None => "some/path".to_string() // TODO: fallback to CWD and user home
        }
    }

    fn get_lines(&self) -> usize {
        match self.matches.opt_str("n") {
            Some(value) => value.parse::<usize>().unwrap(),
            None => DEFAULT_LAST_LINES_SHOWN
        }
    }

    fn assert_file_exists(&self, path: &str) {
        if !Path::new(path).exists() {
            println!("No file exists at provided location `{}`", path);
            process::exit(2);
        }
    }
}

fn build_opts() -> Options {
    let mut opts = Options::new();
    opts.optopt("n", "lines", &format!("Output the last NUM lines. Default is {}.", DEFAULT_LAST_LINES_SHOWN), "NUM");
    opts.optopt("c", "config", "Path to a config file. Defaults to looking in the current directory and user home.", "CONFIG");
    opts.optflag("h", "help", "Print this help menu.");
    opts
}
