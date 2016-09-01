use std::process;
use std::path::Path;
use getopts::{Options, Matches};

pub struct ParsedArgs {
    pub path_to_target_file: String,
    pub path_to_config_file: String,
    pub line_count: usize
}

pub struct ArgsParser {
    args: Vec<String>,
    options: Options
}

impl ArgsParser {
    pub fn new(args: Vec<String>) -> ArgsParser {
        let mut opts = Options::new();
        opts.optopt("n", "lines", "Output the last NUM lines. Default is 10.", "NUM");
        opts.optopt("c", "config", "Path to a config file. Defaults to looking in the current directory and user home.", "CONFIG");
        opts.optflag("h", "help", "Print this help menu.");

        ArgsParser {
            args: args,
            options: opts
        }
    }

    pub fn process(&self) -> ParsedArgs {
        let found_matches = self.options.parse(&self.args[1..]).unwrap();

        if found_matches.opt_present("h") {
            self.print_usage();
            process::exit(0);
        }

        ParsedArgs {
            path_to_target_file: self.get_target(&found_matches),
            path_to_config_file: self.get_config(&found_matches),
            line_count: self.get_lines(&found_matches)
        }
    }

    fn print_usage(&self) {
        let message = format!("Usage: {} FILE [options]", self.args[0]);
        print!("{}", self.options.usage(&message));
    }

    fn get_target(&self, found_matches: &Matches) -> String {
        match found_matches.free.get(0) {
            Some(value) => {
                if !Path::new(value).exists() {
                    println!("No file exists at provided location `{}`", value);
                    process::exit(2);
                }
                value.to_string()
            },
            None => {
                self.print_usage();
                process::exit(1);
            }
        }
    }

    fn get_config(&self, found_matches: &Matches) -> String {
        match found_matches.opt_str("c") {
            Some(value) => value.to_string(),
            None => "some/path".to_string() // TODO: fallback to CWD and user home
        }
    }

    fn get_lines(&self, found_matches: &Matches) -> usize {
        match found_matches.opt_str("n") {
            Some(value) => value.parse::<usize>().unwrap(),
            None => 10
        }
    }
}
