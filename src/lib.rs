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
#[macro_use]
extern crate lazy_static;

extern crate libc;
extern crate regex;
extern crate toml;
extern crate rustc_serialize;
extern crate docopt;
extern crate unicode_width;
extern crate unicode_segmentation;
extern crate ncurses;

#[macro_use]
pub mod macros;
pub mod ext;
pub mod utils;
pub mod core;
pub mod ui;
