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

use std::process;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use getopts::{Options, Matches};
use toml;

use core::filter::Filter;

static DEFAULT_LAST_LINES_SHOWN: usize = 10;
static DEFAULT_MAX_LINES_STORED: usize = 5000;

pub struct Settings {
    pub values: SettingsValues,
    pub config_file: ConfigFile,
}

pub struct SettingsBuilder {
    program_name: String,
    options: Options,
    matches: Matches,
}

impl SettingsBuilder {
    pub fn new(args: Vec<String>) -> SettingsBuilder {
        let opts = build_opts();
        let matches = opts.parse(&args[1..]).unwrap();

        SettingsBuilder {
            program_name: args.get(0).unwrap().clone(),
            options: opts,
            matches: matches,
        }
    }

    pub fn construct(&self) -> Settings {
        if self.matches.opt_present("h") {
            quit!(self.usage());
        }
        let values = SettingsValues {
            path_to_target_file: self.get_target(),
            max_lines_count: self.get_max_lines_count(),
            last_lines_count: self.get_last_lines_count(),
        };

        let config_file = ConfigFile::from_path(self.calculate_config_path());

        Settings {
            values: values,
            config_file: config_file,
        }
    }

    fn usage(&self) -> String {
        let message = format!("Usage: {} FILE [options]", self.program_name);
        self.options.usage(&message)
    }

    fn get_target(&self) -> String {
        match self.matches.free.get(0) {
            Some(value) => {
                self.assert_file_exists(&PathBuf::from(value));
                value.to_string()
            }
            None => {
                quit!(self.usage(), 1);
            }
        }
    }

    fn calculate_config_path(&self) -> PathBuf {
        match self.matches.opt_str("c") {
            Some(value) => {
                let path = PathBuf::from(value);
                self.assert_file_exists(&path);
                path
            }
            None => {
                let mut current_dir_config_path = env::current_dir().unwrap();
                current_dir_config_path.push(".flow");

                let mut home_dir_config_path = env::home_dir().unwrap();
                home_dir_config_path.push(".flow");

                if current_dir_config_path.exists() {
                    current_dir_config_path
                } else if home_dir_config_path.exists() {
                    home_dir_config_path
                } else {
                    quit!("No config file found.", 1);
                }
            }
        }
    }

    fn get_last_lines_count(&self) -> usize {
        match self.matches.opt_str("n") {
            Some(value) => value.parse::<usize>().unwrap(),
            None => DEFAULT_LAST_LINES_SHOWN,
        }
    }

    fn get_max_lines_count(&self) -> usize {
        match self.matches.opt_str("m") {
            Some(value) => value.parse::<usize>().unwrap(),
            None => DEFAULT_MAX_LINES_STORED,
        }
    }

    fn assert_file_exists(&self, path: &PathBuf) {
        if !path.exists() {
            let message = format!("No file exists at provided location `{:?}`", path);
            quit!(message, 2);
        }
    }
}

pub struct SettingsValues {
    pub path_to_target_file: String,
    pub last_lines_count: usize,
    pub max_lines_count: usize,
}

#[derive(RustcDecodable)]
pub struct ConfigFile {
    pub filters: Vec<Filter>,
}

impl ConfigFile {
    pub fn from_path(path: PathBuf) -> ConfigFile {
        let mut file_handle = match File::open(&path) {
            Ok(value) => value,
            Err(message) => {
                let message = format!("{:?} couldn't be opened - {}", path, message);
                quit!(message, 2);
            }
        };

        let contents = &mut String::new();
        let _ = file_handle.read_to_string(contents);

        let parsed_contents = match toml::Parser::new(contents).parse() {
            Some(value) => value,
            None => {
                let message = format!("Provided config file {:?} doesn't have a valid format.",
                                      path);
                quit!(message, 2);
            }
        };

        match toml::decode(toml::Value::Table(parsed_contents)) {
            Some(value) => value,
            None => {
                quit!("Error deserializing config", 2);
            }
        }
    }
}

fn build_opts() -> Options {
    let mut opts = Options::new();
    opts.optopt("n",
                "lines",
                &format!("Output the last NUM lines. Default is {}.",
                         DEFAULT_LAST_LINES_SHOWN),
                "NUM");
    opts.optopt("m",
                "max",
                &format!("Maximum amount of lines to be stored in memory. Default is {}.",
                         DEFAULT_MAX_LINES_STORED),
                "MAX");
    opts.optopt("c",
                "config",
                "Path to a config file. Defaults to looking in the current directory and user \
                 home.",
                "CONFIG");
    opts.optflag("h", "help", "Print this help menu.");
    opts
}
