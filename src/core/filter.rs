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

#[derive(PartialEq)]
pub enum Kind {
    Empty,
    Start,
    Content,
    End,
    StartEnd,
    StartContentEnd,
}

pub enum Match {
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
    Invalid,
}

pub struct Parser {
    pub filter: Filter,
    pub kind: Kind,
    last_match: Match,
}

impl Parser {
    pub fn new(filter: Filter) -> Parser {
        let kind = if filter.start.is_some() && filter.content.is_some() && filter.end.is_some() {
            Kind::StartContentEnd
        } else if filter.start.is_some() && filter.end.is_some() {
            Kind::StartEnd
        } else if filter.content.is_some() {
            Kind::Content
        } else if filter.start.is_some() {
            Kind::Start
        } else if filter.end.is_some() {
            Kind::End
        } else {
            Kind::Empty
        };

        Parser {
            filter: filter,
            kind: kind,
            last_match: Match::Start,
        }
    }

    pub fn matches(&mut self, text: &str) -> ParserResult {
        match self.kind {
            Kind::Start => self.handle_start(text),
            Kind::End => self.handle_end(text),
            Kind::StartEnd => self.handle_start_end(text),
            Kind::StartContentEnd => self.handle_start_content_end(text),
            _ => unreachable!(),
        }
    }

    fn handle_start(&mut self, text: &str) -> ParserResult {
        let start = self.filter.start.as_ref().unwrap();
        let mut result = ParserResult::Match;

        if start.regex.is_match(text) {
            self.last_match = Match::Start;
            result = ParserResult::LastMatch(true);

            if start.has_named_match {
                if !start.is_named_match(text) {
                    result = ParserResult::Invalid;
                }
            }
        }

        result
    }

    fn handle_end(&mut self, text: &str) -> ParserResult {
        match self.last_match {
            Match::Start => {
                if self.filter.end.as_ref().unwrap().is_match(text) {
                    self.last_match = Match::End;

                    ParserResult::Match
                } else {
                    ParserResult::NoMatch
                }
            }
            Match::End => {
                let end = self.filter.end.as_ref().unwrap();

                if end.regex.is_match(text) {
                    if end.is_named_match(text) {
                        ParserResult::LastMatch(true)
                    } else {
                        self.last_match = Match::Start;
                        ParserResult::LastMatch(false)
                    }
                } else {
                    ParserResult::Match
                }
            }
            _ => unreachable!(),
        }
    }

    fn handle_start_end(&mut self, text: &str) -> ParserResult {
        match self.last_match {
            Match::Start => {
                if self.filter.end.as_ref().unwrap().is_match(text) {
                    self.last_match = Match::End;

                    ParserResult::Match
                } else {
                    ParserResult::NoMatch
                }
            }
            Match::End => self.handle_start(text),
            _ => unreachable!(),
        }
    }

    fn handle_start_content_end(&mut self, text: &str) -> ParserResult {
        match self.last_match {
            Match::Start => {
                if self.filter.end.as_ref().unwrap().is_match(text) {
                    self.last_match = Match::End;

                    ParserResult::Match
                } else {
                    ParserResult::NoMatch
                }
            }
            Match::Content => self.handle_start(text),
            Match::End => {
                if self.filter.end.as_ref().unwrap().regex.is_match(text) ||
                   self.filter.start.as_ref().unwrap().regex.is_match(text) {
                    self.last_match = Match::Start;
                    ParserResult::Invalid
                } else {
                    if self.filter.content.as_ref().unwrap().is_match(text) {
                        self.last_match = Match::Content;
                    }
                    ParserResult::Match
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
