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
extern crate flow;

use regex::Regex;
use flow::core::filter::Filter;

#[test]
fn decodes_named_filter() {
    let filter = toml_string_to_filter(r##"
       name = "All"
    "##);

    assert_eq!("All", filter.name);
    assert!(filter.starts_with.is_none());
    assert!(filter.contains.is_none());
    assert!(filter.ends_with.is_none());
}

#[test]
fn decodes_filter_with_content_constraint() {
    let filter = toml_string_to_filter(r##"
       name = "Having content constraints"
       contains = "body value"
    "##);

    assert_eq!("Having content constraints", filter.name);
    assert!(filter.starts_with.is_none());
    let expected_contains = Some(Regex::new("body value").unwrap());
    assert_eq!(expected_contains, filter.contains);
    assert!(filter.ends_with.is_none());
}

#[test]
fn decodes_filter_with_boundary_constraints() {
    let filter = toml_string_to_filter(r##"
       name = "Having boundary constraints"
       starts_with = "start value"
       ends_with = "end value"
    "##);

    assert_eq!("Having boundary constraints", filter.name);

    let expected_starts_with = Some(Regex::new("start value").unwrap());
    let expected_ends_with = Some(Regex::new("end value").unwrap());
    assert_eq!(expected_starts_with, filter.starts_with);
    assert!(filter.contains.is_none());
    assert_eq!(expected_ends_with, filter.ends_with);
}

#[test]
fn decodes_filter_with_boundary_and_content_constraints() {
    let filter = toml_string_to_filter(r##"
       name = "Having many constraints"
       starts_with = "start value"
       contains = "body value"
       ends_with = "end value"
    "##);

    assert_eq!("Having many constraints", filter.name);

    let expected_starts_with = Some(Regex::new("start value").unwrap());
    let expected_contains = Some(Regex::new("body value").unwrap());
    let expected_ends_with = Some(Regex::new("end value").unwrap());
    assert_eq!(expected_starts_with, filter.starts_with);
    assert_eq!(expected_contains, filter.contains);
    assert_eq!(expected_ends_with, filter.ends_with);
}

#[test]
fn having_no_constraints_matches_everything() {
    let mut filter = toml_string_to_filter(r##"
       name = "All"
    "##);

    for value in vec!["", "lorem ipsum", "こんにちは！"] {
        assert!(filter.is_match(&value.to_string()));
    }
}

#[test]
fn having_content_constraint_matches_against_provided_value() {
    let mut filter = toml_string_to_filter(r##"
       name = "All"
       contains = "(?i)or"
    "##);

    let values = vec!["Lorem ipsum".to_string(), "Legislature".to_string(), "Folklore".to_string()];

    let actual = values.iter().filter(|&x| filter.is_match(&x)).collect::<Vec<_>>();
    let expected = vec!["Lorem ipsum", "Folklore"];

    assert_eq!(expected, actual);
}

#[test]
fn having_boundary_constraints_matches_everything_between_the_two() {
    let mut filter = toml_string_to_filter(r##"
       name = "Story"
       starts_with = "Once upon a time"
       ends_with = "The end"
    "##);

    let values = vec![
        "And so the story starts",
        "Once upon a time,",
        "There was a castle",
        "and if I recall correctly",
        "there was also a dragon",
        "The end",
        "And so the test starts",
        "Once upon a time,",
        "There was a programmer",
        "and was also a laptop.",
        "The end",
    ].into_iter().map(|x| x.to_string());

    let actual = values.rev().filter(|ref x| filter.is_match(&x)).collect::<Vec<_>>();
    let expected = vec![
        "Once upon a time,",
        "There was a castle",
        "and if I recall correctly",
        "there was also a dragon",
        "The end",
        "Once upon a time,",
        "There was a programmer",
        "and was also a laptop.",
        "The end",
    ].into_iter().rev().map(|x| x.to_string()).collect::<Vec<_>>();

    assert_eq!(expected, actual);
}

#[test]
fn having_boundary_and_content_constraints_matches_content_between_the_end_points() {
    let mut filter = toml_string_to_filter(r##"
       name = "Story"
       starts_with = "Once upon a time"
       contains = "(?i)there was"
       ends_with = "The end"
    "##);

    let values = vec![
        "And so the story starts",
        "Once upon a time,",
        "There was a castle",
        "and if I recall correctly",
        "there was also a dragon",
        "The end",
        "And so the test starts",
        "Once upon a time,",
        "There was a programmer",
        "and was also a laptop.",
        "The end",
    ].into_iter().map(|x| x.to_string());

    let actual = values.rev().filter(|ref x| filter.is_match(&x)).collect::<Vec<_>>();
    let expected = vec![
        "Once upon a time,",
        "There was a castle",
        "there was also a dragon",
        "The end",
        "Once upon a time,",
        "There was a programmer",
        "The end",
    ].into_iter().rev().map(|x| x.to_string()).collect::<Vec<_>>();

    assert_eq!(expected, actual);
}

fn toml_string_to_filter(contents: &str) -> Filter {
    let parsed_contents = toml::Parser::new(contents).parse().unwrap();
    toml::decode(toml::Value::Table(parsed_contents)).unwrap()
}
