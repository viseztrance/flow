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

use utils::args::Args;
use utils::config_file::ConfigFile;
use core::filter::Filter;

static DEFAULT_LAST_LINES_SHOWN: usize = 10;
static DEFAULT_MAX_LINES_STORED: usize = 3000;

pub struct Settings {
    pub path_to_target_file: String,
    pub last_lines_count: usize,
    pub max_lines_count: usize,
    pub filters: Vec<Filter>,
}

impl Settings {
    pub fn from_args(args: Args) -> Settings {
        let config = ConfigFile::from_path(args.get_config())
            .unwrap_or(ConfigFile::from_current_dir()
                .unwrap_or(ConfigFile::from_home_dir().unwrap_or_else(ConfigFile::default)));

        assert_quit!(!config.filters.is_empty(),
                     "At least one filter needs to be defined.");

        Settings {
            path_to_target_file: args.get_target(),
            last_lines_count: args.flag_lines.unwrap_or(DEFAULT_LAST_LINES_SHOWN),
            max_lines_count: args.flag_max.unwrap_or(DEFAULT_MAX_LINES_STORED),
            filters: config.filters,
        }
    }

    pub fn menu_item_names(&self) -> Vec<String> {
        self.filters
            .iter()
            .map(|tab| tab.name.clone())
            .collect()
    }
}
