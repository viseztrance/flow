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
use toml;

use core::filter::Filter;

static DEFAULT_LAST_LINES_SHOWN: usize = 10;
static DEFAULT_MAX_LINES_STORED: usize = 3000;

#[derive(Debug, RustcDecodable)]
pub struct Args {
    arg_input: Option<String>,
    flag_config: Option<String>,
    flag_lines: Option<usize>,
    flag_max: Option<usize>,
}

pub struct Settings {
    pub path_to_target_file: String,
    pub last_lines_count: usize,
    pub max_lines_count: usize,
    pub filters: Vec<Filter>,
}

impl Settings {
    pub fn from_args(args: Args) -> Settings {
        let config = ConfigFile::from_path(determine_config_path(args.flag_config));
        let target = args.arg_input.unwrap_or_else(|| {
            quit!("No input file provided");
        });
        assert_file_exists(&PathBuf::from(&target));

        Settings {
            path_to_target_file: target,
            last_lines_count: args.flag_lines.unwrap_or(DEFAULT_LAST_LINES_SHOWN),
            max_lines_count: args.flag_max.unwrap_or(DEFAULT_MAX_LINES_STORED),
            filters: config.filters,
        }
    }
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

fn determine_config_path(path: Option<String>) -> PathBuf {
    match path {
        Some(value) => {
            let pathbuf = PathBuf::from(value);
            assert_file_exists(&pathbuf);
            pathbuf
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

fn assert_file_exists(path: &PathBuf) {
    if !path.exists() {
        let message = format!("No file exists at provided location `{:?}`", path);
        quit!(message, 2);
    }
}
