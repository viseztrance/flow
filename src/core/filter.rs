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
use regex::Regex;
use rustc_serialize::{Decodable, Decoder};

#[derive(Clone)]
pub enum Match {
    StartsWith,
    Contains,
    EndsWith,
}

#[derive(Clone)]
pub enum Kind {
    Content,
    StartEnd,
    StartContentEnd,
}

#[derive(Clone)]
pub struct Filter {
    pub name: String,
    pub starts_with: Option<Regex>,
    pub contains: Option<Regex>,
    pub ends_with: Option<Regex>,
    pub last_match: Match,
    pub kind: Kind,
}

impl Filter {
    fn new(name: String,
           starts_with: Option<Regex>,
           contains: Option<Regex>,
           ends_with: Option<Regex>)
           -> Filter {

        let kind = determine_kind(&starts_with, &contains, &ends_with);
        let last_match = match kind {
            Kind::Content => Match::Contains,
            _ => Match::StartsWith,
        };

        Filter {
            name: name,
            starts_with: starts_with,
            contains: contains,
            ends_with: ends_with,
            kind: kind,
            last_match: last_match,
        }
    }

    pub fn is_match(&mut self, text: &str) -> bool {
        match self.kind {
            Kind::Content => self.handle_content(text),
            Kind::StartContentEnd => self.handle_start_content_end(text),
            Kind::StartEnd => self.handle_start_end(text),
        }
    }

    fn handle_content(&self, text: &str) -> bool {
        match self.contains {
            Some(ref value) => value.is_match(text),
            None => true,
        }
    }

    fn handle_start_content_end(&mut self, text: &str) -> bool {
        match self.last_match {
            Match::StartsWith => {
                let result = self.ends_with.as_ref().unwrap().is_match(text);
                if result {
                    self.last_match = Match::EndsWith;
                }
                result
            }
            Match::Contains => {
                let mut result = self.contains.as_ref().unwrap().is_match(text);
                if result {
                    return result;
                }

                result = self.starts_with.as_ref().unwrap().is_match(text);
                if result {
                    self.last_match = Match::StartsWith
                }
                result
            }
            Match::EndsWith => {
                let result = self.contains.as_ref().unwrap().is_match(text);
                if result {
                    self.last_match = Match::Contains;
                }
                result
            }
        }
    }

    fn handle_start_end(&mut self, text: &str) -> bool {
        match self.last_match {
            Match::StartsWith => {
                let result = self.ends_with.as_ref().unwrap().is_match(text);
                if result {
                    self.last_match = Match::EndsWith;
                }
                result
            }
            Match::EndsWith => {
                let result = self.starts_with.as_ref().unwrap().is_match(text);
                if result {
                    self.last_match = Match::StartsWith;
                }
                true
            }
            _ => unreachable!("Unexpected previous match found!"),
        }
    }
}

impl Decodable for Filter {
    fn decode<D: Decoder>(d: &mut D) -> Result<Filter, D::Error> {
        d.read_struct("Filter", 2, |d| {
            let name = try!(d.read_struct_field("name", 0, |d| d.read_str()));
            let starts_with = read_struct_field(d, "starts_with", 1);
            let contains = read_struct_field(d, "contains", 1);
            let ends_with = read_struct_field(d, "ends_with", 1);

            let filter = Filter::new(name, starts_with, contains, ends_with);

            Ok(filter)
        })
    }
}

fn determine_kind(starts_with: &Option<Regex>,
                  contains: &Option<Regex>,
                  ends_with: &Option<Regex>)
                  -> Kind {
    if starts_with.is_none() && contains.is_none() && ends_with.is_none() {
        Kind::Content
    } else if contains.is_some() {
        if starts_with.is_some() {
            assert_quit!(ends_with.is_some(),
                         "Expected an `ends_with` value to be found alongside `starts_with`.");

            Kind::StartContentEnd
        } else {
            assert_quit!(ends_with.is_none(),
                         "Expected a `starts_with` value to be found alongside `ends_with`.");

            Kind::Content
        }
    } else {
        assert_quit!(starts_with.is_some(),
                     "Expected a `starts_with` value to be found alongside `ends_with`.");

        assert_quit!(ends_with.is_some(),
                     "Expected an `ends_with` value to be found alongside `starts_with`.");

        Kind::StartEnd
    }
}

fn read_struct_field<D: Decoder>(decoder: &mut D, name: &str, idx: usize) -> Option<Regex> {
    match decoder.read_struct_field(name, idx, |d| d.read_str()) {
        Ok(val) => Some(Regex::new(&val).unwrap()),
        Err(_) => None,
    }
}
