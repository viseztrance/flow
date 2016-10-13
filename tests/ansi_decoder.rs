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

extern crate flow;

use flow::utils::ansi_decoder::AnsiStr;

#[test]
fn detects_ansi_escape_sequence() {
    let text_with_ansi = "[1m[36mHello[0m,[1m ncurses![0m";
    assert!(text_with_ansi.has_ansi_escape_sequence());

    let text_without_ansi = "Hello, ncurses!";
    assert!(!text_without_ansi.has_ansi_escape_sequence());
}

#[test]
fn strip_ansi_colors_from_ansi_string() {
    let expected = "Hello, ncurses!".to_string();
    let actual = "[1m[36mHello[0m,[1m ncurses![0m";

    assert_eq!(expected, actual.strip_ansi());
}

#[test]
fn strip_ansi_colors_from_normal_string() {
    let expected = "Hello, ncurses!".to_string();
    let actual = "Hello, ncurses!";

    assert_eq!(expected, actual.strip_ansi());
}

#[test]
fn strip_ansi_colors_from_fake_ansi_string() {
    let expected = "^[[1m^[[36mHello^[[0m,^[[1m ncurses!^[[0m".to_string();
    let actual = "^[[1m^[[36mHello^[[0m,^[[1m ncurses!^[[0m";

    assert_eq!(expected, actual.strip_ansi());
}

#[test]
fn breaks_text_into_ncurses_components() {
    let text_with_ansi = "[1m[36mHello[0m,[1m ncurses![0m";
    let components = text_with_ansi.to_components();
    assert_eq!(8, components.items.len());

    let text_with_unknown_ansi = "[1m[99mHello[0m,[1m ncurses![0m";
    let components = text_with_unknown_ansi.to_components();
    assert_eq!(7, components.items.len());
}
