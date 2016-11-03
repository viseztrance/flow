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
use std::path::PathBuf;

use utils::config_file::ConfigFile;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug, RustcDecodable)]
pub struct Args {
    pub flag_config: Option<String>,
    pub flag_max: Option<usize>,
    pub flag_lines: Option<usize>,
    arg_input: Option<String>,
    flag_init: Option<String>,
    flag_version: bool,
}

impl Args {
    pub fn process<F>(self, callback: F)
        where F: Fn(Args)
    {
        if self.flag_init.is_some() {
            self.write_config();
        } else if self.flag_version {
            self.display_version();
        }

        callback(self);
    }

    pub fn write_config(&self) {
        let path = self.flag_init.as_ref().unwrap();

        ConfigFile::write_sample(path);

        let message = format!("Wrote config file at `{}`.", path);
        quit!(message);
    }

    fn display_version(&self) {
        let message = format!("flow version {}", VERSION);
        quit!(message);
    }

    pub fn get_target(&self) -> String {
        let target = self.arg_input.as_ref().unwrap_or_else(|| {
            critical_quit!("No input file provided");
        });
        assert_file_exists(&PathBuf::from(target));

        target.to_string()
    }

    pub fn get_config(&self) -> PathBuf {
        if let Some(ref value) = self.flag_config {
            let pathbuf = PathBuf::from(value);
            assert_file_exists(&pathbuf);
            pathbuf
        } else {
            PathBuf::from("")
        }
    }
}

fn assert_file_exists(path: &PathBuf) {
    if !path.exists() {
        let message = format!("No file exists at provided location `{:?}`", path);
        critical_quit!(message);
    }
}
