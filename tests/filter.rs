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

extern crate toml;
extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate flow;

use regex::Regex;
use flow::core::filter::Filter;
use flow::core::line::{Line, Parser};

lazy_static! {
    static ref LINES: Vec<Line> = vec![
        "Setting database",
        "Started POST",
        "dolor needle",
        "sit amen mel",
        "Completed 200",
        "Started GET",
        "quo natum",
        "mel elit needle",
        "Completed 200",
        "Started GET",
        "graece ceteros",
        "neglegentur id",
        "Completed 200",
        "Started GET",
        "electram needle",
        "sit amen",
        "Completed 404",
        "Started GET",
        "concludaturque mel",
        "sit ea NEEDLE ignota",
        "Completed 200",
    ].iter().map(|x| Line::new(x.to_string())).collect::<Vec<_>>();
}

#[test]
fn decodes_named_filter() {
    let filter = toml_string_to_filter(r##"
       name = "All"
    "##);

    assert_eq!("All", filter.name);
    assert!(filter.start.is_none());
    assert!(filter.content.is_none());
    assert!(filter.end.is_none());
}

#[test]
fn decodes_filter_with_content_constraint() {
    let filter = toml_string_to_filter(r##"
       name = "Having content constraints"
       contains = "body value"
    "##);

    assert_eq!("Having content constraints", filter.name);
    assert!(filter.start.is_none());
    let expected_contains = Some(Regex::new("body value").unwrap());
    assert_eq!(expected_contains, filter.content);
    assert!(filter.end.is_none());
}

#[test]
fn decodes_filter_with_boundary_constraints() {
    let filter = toml_string_to_filter(r##"
       name = "Having boundary constraints"
       starts_with = "start marker"
       ends_with = "end marker"
    "##);

    let expected_starts_with = Regex::new("start marker").unwrap();
    let expected_ends_with = Regex::new("end marker").unwrap();

    assert_eq!("Having boundary constraints", filter.name);
    assert!(filter.content.is_none());
    assert_eq!(expected_starts_with, filter.start.unwrap().regex);
    assert_eq!(expected_ends_with, filter.end.unwrap().regex);
}

#[test]
fn decodes_filter_with_boundary_match_constraints() {
    let filter = toml_string_to_filter(r##"
       name = "Having many constraints"
       contains = "body value"
       starts_with = "(?P<matching>start) body value"
       ends_with = "(?P<other>end) body value"
    "##);

    assert_eq!("Having many constraints", filter.name);

    let expected_body_content = Regex::new("body value").unwrap();
    assert!(filter.start.unwrap().has_named_match);
    assert_eq!(expected_body_content, filter.content.unwrap());
    assert!(!filter.end.unwrap().has_named_match);
}

#[test]
fn having_no_constraints_matches_everything() {
    let filter = toml_string_to_filter(r##"
       name = "All"
    "##);

    let lines = vec![Line::new("".to_string()),
                     Line::new("lorem ipsum".to_string()),
                     Line::new("こんにちは！".to_string())];
    assert_eq!(3, lines.iter().parse(filter).count());
}

#[test]
fn having_content_constraint_matches_against_provided_value() {
    let filter = toml_string_to_filter(r##"
       name = "All"
       contains = "(?i)or"
    "##);

    let lines = vec![Line::new("Lorem ipsum".to_string()),
                     Line::new("Legislature".to_string()),
                     Line::new("Folklore".to_string())];
    let actual = lines.iter()
        .parse(filter)
        .map(|line| line.content_without_ansi.clone())
        .collect::<Vec<_>>();
    let expected = vec!["Folklore", "Lorem ipsum"];
    assert_eq!(expected, actual);
}

#[test]
fn filters_entries_by_start_boundary() {
    let filter = toml_string_to_filter(r##"
       name = "GET requests"
       starts_with = "Started (?P<matching>GET)?"
    "##);

    let expected = vec!["Completed 200",
                        "sit ea NEEDLE ignota",
                        "concludaturque mel",
                        "Started GET",
                        "Completed 404",
                        "sit amen",
                        "electram needle",
                        "Started GET",
                        "Completed 200",
                        "neglegentur id",
                        "graece ceteros",
                        "Started GET",
                        "Completed 200",
                        "mel elit needle",
                        "quo natum",
                        "Started GET",];
    assert_line_content(filter, expected);
}

#[test]
fn filters_entries_by_main_content_and_start_boundary() {
    let filter = toml_string_to_filter(r##"
       name = "GET requests"
       starts_with = "Started (?P<matching>GET)?"
       contains = "mel"
    "##);

    let expected = vec!["Completed 200",
                        "sit ea NEEDLE ignota",
                        "concludaturque mel",
                        "Started GET",
                        "Completed 200",
                        "mel elit needle",
                        "quo natum",
                        "Started GET",];
    assert_line_content(filter, expected);
}

#[test]
fn filters_entries_by_end_boundary() {
    let filter = toml_string_to_filter(r##"
       name = "GET requests"
       ends_with = "Completed (?P<matching>200)?"
    "##);

    let expected = vec!["Completed 200",
                        "sit ea NEEDLE ignota",
                        "concludaturque mel",
                        "Started GET",
                        "Completed 200",
                        "neglegentur id",
                        "graece ceteros",
                        "Started GET",
                        "Completed 200",
                        "mel elit needle",
                        "quo natum",
                        "Started GET",
                        "Completed 200",
                        "sit amen mel",
                        "dolor needle",
                        "Started POST",
                        "Setting database",];
    assert_line_content(filter, expected);
}

#[test]
fn filters_entries_by_main_content_and_end_boundary() {
    let filter = toml_string_to_filter(r##"
       name = "GET requests"
       contains = "sit"
       ends_with = "Completed (?P<matching>200)?"
    "##);

    let expected = vec!["Completed 200",
                        "sit ea NEEDLE ignota",
                        "concludaturque mel",
                        "Started GET",
                        "Completed 200",
                        "sit amen mel",
                        "dolor needle",
                        "Started POST",
                        "Setting database",];
    assert_line_content(filter, expected);
}

#[test]
fn filters_entries_by_boundary_with_matcher() {
    let filter = toml_string_to_filter(r##"
       name = "Having many constraints"
       starts_with = "Started (?P<matching>GET)?"
       ends_with = "Completed (?P<matching>200)?"
    "##);

    let expected = vec!["Completed 200",
                        "sit ea NEEDLE ignota",
                        "concludaturque mel",
                        "Started GET",
                        "Completed 200",
                        "neglegentur id",
                        "graece ceteros",
                        "Started GET",
                        "Completed 200",
                        "mel elit needle",
                        "quo natum",
                        "Started GET",];
    assert_line_content(filter, expected);
}

#[test]
fn filters_entries_by_boundary_without_matcher() {
    let filter = toml_string_to_filter(r##"
       name = "GET requests"
       starts_with = "Started (?P<matching>GET)?"
       ends_with = "Completed"
    "##);

    let expected = vec!["Completed 200",
                        "sit ea NEEDLE ignota",
                        "concludaturque mel",
                        "Started GET",
                        "Completed 404",
                        "sit amen",
                        "electram needle",
                        "Started GET",
                        "Completed 200",
                        "neglegentur id",
                        "graece ceteros",
                        "Started GET",
                        "Completed 200",
                        "mel elit needle",
                        "quo natum",
                        "Started GET",];
    assert_line_content(filter, expected);
}

#[test]
fn filters_entries_by_main_content_and_boundary_with_matcher() {
    let filter = toml_string_to_filter(r##"
       name = "Succesful GET requests"
       contains = "(?i)needle"
       starts_with = "Started (?P<matching>GET)?"
       ends_with = "Completed (?P<matching>200)?"
    "##);

    let expected = vec!["Completed 200",
                        "sit ea NEEDLE ignota",
                        "concludaturque mel",
                        "Started GET",
                        "Completed 200",
                        "mel elit needle",
                        "quo natum",
                        "Started GET",];
    assert_line_content(filter, expected);
}

#[test]
fn filters_entries_by_main_content_and_boundary_without_matcher() {
    let filter = toml_string_to_filter(r##"
       name = "GET requests"
       contains = "(?i)needle"
       starts_with = "Started (?P<matching>GET)?"
       ends_with = "Completed"
    "##);

    let expected = vec!["Completed 200",
                        "sit ea NEEDLE ignota",
                        "concludaturque mel",
                        "Started GET",
                        "Completed 404",
                        "sit amen",
                        "electram needle",
                        "Started GET",
                        "Completed 200",
                        "mel elit needle",
                        "quo natum",
                        "Started GET",];
    assert_line_content(filter, expected);
}

fn assert_line_content(filter: Filter, expected: Vec<&str>) {
    let actual = LINES.iter()
        .parse(filter)
        .map(|line| line.content_without_ansi.clone())
        .collect::<Vec<_>>();
    assert_eq!(expected, actual);
}

fn toml_string_to_filter(contents: &str) -> Filter {
    let parsed_contents = toml::Parser::new(contents).parse().unwrap();
    toml::decode(toml::Value::Table(parsed_contents)).unwrap()
}
