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

use regex::Regex;
use rustc_serialize::{Decodable, Decoder};

#[derive(Clone, PartialEq, Debug)]
pub enum Constraint {
    Start,
    Content,
    End,
}

#[derive(Clone)]
pub struct BoundaryFilter {
    pub regex: Regex,
    pub has_named_match: bool,
}

impl BoundaryFilter {
    fn is_match(&self, text: &str) -> bool {
        if self.has_named_match {
            self.is_named_match(text)
        } else {
            self.regex.is_match(text)
        }
    }

    fn is_named_match(&self, text: &str) -> bool {
        if let Some(captures) = self.regex.captures(text) {
            captures.name("matching").is_some()
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct Filter {
    pub name: String,
    pub content: Option<Regex>,
    pub start: Option<BoundaryFilter>,
    pub end: Option<BoundaryFilter>,
}

pub enum ParserResult {
    Match,
    NoMatch,
    LastMatch(bool),
    Invalid(bool),
}

pub struct Parser {
    pub filter: Filter,
    pub constraints: Vec<Constraint>,
    active_constraint: Constraint,
    first_match: bool,
}

impl Parser {
    pub fn new(filter: Filter) -> Parser {
        Parser {
            active_constraint: Constraint::Content,
            constraints: filter.determine_constraints(),
            filter: filter,
            first_match: true,
        }
    }

    pub fn matches(&mut self, text: &str) -> ParserResult {
        if self.constraints == vec![Constraint::End] {
            self.end_constraint_parser(text).parse()
        } else if self.constraints == vec![Constraint::End, Constraint::Content] {
            self.content_end_constraints_parser(text).parse()
        } else {
            self.constraints_parser(text).parse()
        }
    }

    fn end_constraint_parser<'a>(&'a mut self, text: &'a str) -> EndConstraintParser<'a> {
        EndConstraintParser {
            text: text,
            parser: self,
        }
    }

    fn content_end_constraints_parser<'a>(&'a mut self,
                                          text: &'a str)
                                          -> ContentEndConstraintsParser<'a> {
        ContentEndConstraintsParser {
            text: text,
            parser: self,
        }
    }

    fn constraints_parser<'a>(&'a mut self, text: &'a str) -> ConstraintsParser<'a> {
        ConstraintsParser {
            text: text,
            parser: self,
        }
    }

    pub fn assume_found_matches(&self) -> bool {
        self.constraints == [Constraint::End] ||
        self.constraints == [Constraint::End, Constraint::Content] &&
        self.active_constraint == Constraint::Content
    }

    fn next_constraint(&self) -> &Constraint {
        let index = self.constraints.iter().position(|c| *c == self.active_constraint).unwrap_or(0);
        self.constraints.get(index + 1).unwrap_or(&self.constraints[0])
    }
}

struct ConstraintsParser<'a> {
    text: &'a str,
    parser: &'a mut Parser,
}

impl<'a> ConstraintsParser<'a> {
    fn parse(&mut self) -> ParserResult {
        match *self.parser.next_constraint() {
            Constraint::Start => self.handle_start(),
            Constraint::Content => self.handle_content(),
            Constraint::End => self.handle_end(),
        }
    }

    fn handle_start(&mut self) -> ParserResult {
        let start = self.parser.filter.start.as_ref().unwrap();
        let mut result = ParserResult::Match;

        if start.regex.is_match(self.text) {
            self.parser.active_constraint = Constraint::Start;
            result = ParserResult::LastMatch(true);

            if start.has_named_match {
                if !start.is_named_match(self.text) {
                    result = ParserResult::Invalid(false);
                }
            }
        }

        result
    }

    fn handle_content(&mut self) -> ParserResult {
        if self.parser.filter.is_partial_match(Constraint::Start, self.text) ||
           self.parser.filter.is_partial_match(Constraint::End, self.text) {
            self.parser.active_constraint = self.parser.constraints.iter().last().unwrap().clone();
            ParserResult::Invalid(false)
        } else {
            if self.parser.filter.content.as_ref().unwrap().is_match(self.text) {
                self.parser.active_constraint = Constraint::Content;
            }

            ParserResult::Match
        }
    }

    fn handle_end(&mut self) -> ParserResult {
        if self.parser.filter.end.as_ref().unwrap().is_match(self.text) {
            self.parser.active_constraint = Constraint::End;

            ParserResult::Match
        } else {
            ParserResult::NoMatch
        }
    }
}

struct EndConstraintParser<'a> {
    text: &'a str,
    parser: &'a mut Parser,
}

impl<'a> EndConstraintParser<'a> {
    fn parse(&mut self) -> ParserResult {
        if self.parser.active_constraint == Constraint::End {
            self.handle_normal_occurrence()
        } else {
            self.handle_first_occurrence()
        }
    }

    fn handle_first_occurrence(&mut self) -> ParserResult {
        if self.parser.filter.end.as_ref().unwrap().is_match(self.text) {
            self.parser.active_constraint = Constraint::End;

            ParserResult::Match
        } else {
            ParserResult::NoMatch
        }
    }

    fn handle_normal_occurrence(&mut self) -> ParserResult {
        let end = self.parser.filter.end.as_ref().unwrap();

        if end.regex.is_match(self.text) {
            if end.is_named_match(self.text) {
                ParserResult::LastMatch(true)
            } else {
                self.parser.active_constraint = Constraint::Start;
                ParserResult::LastMatch(false)
            }
        } else {
            ParserResult::Match
        }
    }
}

struct ContentEndConstraintsParser<'a> {
    text: &'a str,
    parser: &'a mut Parser,
}

impl<'a> ContentEndConstraintsParser<'a> {
    fn parse(&mut self) -> ParserResult {
        match *self.parser.next_constraint() {
            Constraint::Start => unreachable!(),
            Constraint::Content => self.handle_content(),
            Constraint::End => {
                if self.parser.first_match {
                    self.handle_first_end_occurrence()
                } else {
                    self.handle_normal_end_occurrence()
                }
            }
        }
    }

    fn handle_content(&mut self) -> ParserResult {
        let end = self.parser.filter.end.as_ref().unwrap();

        if end.regex.is_match(self.text) {
            let is_match = end.is_named_match(self.text);
            if !is_match {
                self.parser.first_match = true;
                self.parser.active_constraint = Constraint::Content;
            }
            ParserResult::Invalid(is_match)
        } else {
            if self.parser.filter.content.as_ref().unwrap().is_match(self.text) {
                self.parser.active_constraint = Constraint::Content;
            }
            ParserResult::Match
        }
    }

    fn handle_first_end_occurrence(&mut self) -> ParserResult {
        if self.parser.filter.end.as_ref().unwrap().is_match(self.text) {
            self.parser.first_match = false;
            self.parser.active_constraint = Constraint::End;

            ParserResult::Match
        } else {
            ParserResult::NoMatch
        }
    }

    fn handle_normal_end_occurrence(&mut self) -> ParserResult {
        let end = self.parser.filter.end.as_ref().unwrap();

        if end.regex.is_match(self.text) {
            if end.is_named_match(self.text) {
                self.parser.active_constraint = Constraint::End;
                ParserResult::LastMatch(true)
            } else {
                self.parser.first_match = true;
                ParserResult::LastMatch(false)
            }
        } else {
            ParserResult::Match
        }
    }
}

impl Filter {
    fn determine_constraints(&self) -> Vec<Constraint> {
        let mut constraints = vec![];

        if self.end.is_some() {
            constraints.push(Constraint::End);
        }

        if self.content.is_some() {
            constraints.push(Constraint::Content);
        }

        if self.start.is_some() {
            constraints.push(Constraint::Start);
        }

        constraints
    }

    fn is_partial_match(&self, constraint: Constraint, text: &str) -> bool {
        match constraint {
            Constraint::Start => {
                match self.start {
                    Some(ref start) => start.regex.is_match(text),
                    None => false,
                }
            }
            Constraint::Content => {
                match self.content {
                    Some(ref content) => content.is_match(text),
                    None => false,
                }
            }
            Constraint::End => {
                match self.end {
                    Some(ref end) => end.regex.is_match(text),
                    None => false,
                }
            }
        }
    }
}

impl Decodable for Filter {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Filter, D::Error> {
        decoder.read_struct("Filter", 2, |d| {
            let filter = Filter {
                name: try!(d.read_struct_field("name", 0, |d| d.read_str())),
                content: field_to_regex(d, "contains", 1),
                start: regex_to_boundary(field_to_regex(d, "starts_with", 2)),
                end: regex_to_boundary(field_to_regex(d, "ends_with", 3)),
            };

            Ok(filter)
        })
    }
}

fn field_to_regex<D: Decoder>(decoder: &mut D, name: &str, idx: usize) -> Option<Regex> {
    match decoder.read_struct_field(name, idx, |d| d.read_str()) {
        Ok(val) => Some(Regex::new(&val).unwrap()),
        Err(_) => None,
    }
}

fn regex_to_boundary(regex: Option<Regex>) -> Option<BoundaryFilter> {
    match regex {
        Some(val) => {
            Some(BoundaryFilter {
                has_named_match: val.capture_names().any(|c| c == Some("matching")),
                regex: val,
            })
        }
        None => None,
    }
}
