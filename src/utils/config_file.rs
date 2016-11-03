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

use std::path::PathBuf;
use std::{env, process};
use std::fs::{self, File};
use std::io::{Read, Write};
use toml;

use core::filter::Filter;

const SAMPLE: &'static [u8] = include_bytes!("../etc/sample-config.toml");
const DEFAULT: &'static str = include_str!("../etc/default-config.toml");

#[derive(RustcDecodable)]
pub struct ConfigFile {
    pub filters: Vec<Filter>,
}

impl ConfigFile {
    pub fn from_path(path: PathBuf) -> Option<ConfigFile> {
        if !path.exists() {
            return None;
        }

        let mut file_handle = match File::open(&path) {
            Ok(value) => value,
            Err(message) => {
                let message = format!("{:?} couldn't be opened - {}", path, message);
                critical_quit!(message);
            }
        };

        let contents = &mut String::new();
        let _ = file_handle.read_to_string(contents);

        Some(ConfigFile::new(contents))
    }

    pub fn from_current_dir() -> Option<ConfigFile> {
        let mut path = env::current_dir().unwrap();
        path.push(".flow");

        ConfigFile::from_path(path)
    }

    pub fn from_home_dir() -> Option<ConfigFile> {
        let mut path = env::home_dir().unwrap();
        path.push(".flow");

        ConfigFile::from_path(path)
    }

    pub fn default() -> ConfigFile {
        ConfigFile::new(DEFAULT)
    }

    pub fn write_sample(path: &PathBuf) {
        assert_quit!(!path.exists(),
                     format!("{:?} already exists.", fs::canonicalize(path).unwrap()));

        let mut file_handle = match File::create(path) {
            Ok(value) => value,
            Err(message) => {
                let message = format!("{} couldn't be created - {}", path.display(), message);
                critical_quit!(message);
            }
        };

        let _ = file_handle.write(SAMPLE);
    }

    fn new(contents: &str) -> ConfigFile {
        let parsed_contents = match toml::Parser::new(contents).parse() {
            Some(value) => value,
            None => {
                critical_quit!("Provided config file doesn't have a valid format.");
            }
        };

        match toml::decode(toml::Value::Table(parsed_contents)) {
            Some(value) => value,
            None => {
                critical_quit!("Error deserializing config");
            }
        }
    }
}
