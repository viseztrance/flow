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

use std::cmp::max;
use std::collections::VecDeque;
use std::iter::{Rev, DoubleEndedIterator};

use unicode_width::UnicodeWidthStr;

use core::filter::{Filter, Parser as FilterParser, Kind as FilterKind,
                   ParserResult as FilterParserResult};
use utils::ansi_decoder::{ComponentCollection, AnsiStr};

pub struct Line {
    pub content_without_ansi: String,
    pub components: Option<ComponentCollection>,
    pub width: usize,
}

impl Line {
    pub fn new(content: String) -> Line {
        let has_ansi = content.has_ansi_escape_sequence();

        let (content_without_ansi, components) = if has_ansi {
            (content.strip_ansi(), Some(content.to_components()))
        } else {
            (content, None)
        };

        Line {
            width: content_without_ansi.width(),
            content_without_ansi: content_without_ansi,
            components: components,
        }
    }

    pub fn guess_height(&self, container_width: usize) -> usize {
        max(1,
            (self.width as f32 / container_width as f32).ceil() as usize)
    }

    pub fn matches_for(&self, text: &str) -> Vec<(usize, &str)> {
        self.content_without_ansi.match_indices(text).collect()
    }

    pub fn contains(&self, text: &str) -> bool {
        self.content_without_ansi.contains(text)
    }
}

pub struct LineCollection {
    pub entries: VecDeque<Line>,
    capacity: usize,
}

impl LineCollection {
    pub fn new(capacity: usize) -> LineCollection {
        LineCollection {
            entries: VecDeque::new(),
            capacity: capacity,
        }
    }

    fn clear_excess(&mut self) {
        while self.entries.len() > self.capacity {
            self.entries.pop_front();
        }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    fn add(&mut self, item: String) {
        self.entries.push_back(Line::new(item));
    }
}

impl Extend<String> for LineCollection {
    fn extend<T: IntoIterator<Item = String>>(&mut self, iter: T) {
        for item in iter {
            self.add(item);
        }

        self.clear_excess();
    }
}


pub struct ParserState<'a, I>
    where I: DoubleEndedIterator<Item = &'a Line>
{
    iterator: I,
    parser: FilterParser,
    pending: Vec<&'a Line>,
}

impl<'a, I> ParserState<'a, I>
    where I: DoubleEndedIterator<Item = &'a Line>
{
    fn handle_empty(&mut self) -> Option<I::Item> {
        self.iterator.next()
    }

    fn handle_content(&mut self) -> Option<I::Item> {
        let matcher = self.parser.filter.content.as_ref().unwrap();

        (&mut self.iterator).filter(|line| matcher.is_match(&line.content_without_ansi)).next()
    }

    fn handle_boundaries(&mut self) -> Option<I::Item> {
        if self.pending.is_empty() {
            // There are no invalid pending entries for `End` filters
            let mut match_found = self.parser.kind == FilterKind::End;

            for line in &mut self.iterator {
                match self.parser.matches(&line.content_without_ansi) {
                    FilterParserResult::Match => self.pending.push(line),
                    FilterParserResult::LastMatch(append) => {
                        match_found = true;
                        if append {
                            self.pending.push(line);
                        }
                        break;
                    }
                    FilterParserResult::Invalid => self.pending.clear(),
                    FilterParserResult::NoMatch => {}
                }
            }
            if !match_found {
                return None;
            }

            self.pending.reverse();
        }

        self.pending.pop()
    }
}

pub trait Parser<'a>: Iterator<Item = &'a Line> {
    fn parse(self, filter: Filter) -> ParserState<'a, Rev<Self>>
        where Self: DoubleEndedIterator + Sized;
}

impl<'a, I> Parser<'a> for I
    where I: Iterator<Item = &'a Line>
{
    fn parse(self, filter: Filter) -> ParserState<'a, Rev<Self>>
        where Self: DoubleEndedIterator + Sized
    {
        ParserState {
            iterator: self.rev(),
            pending: vec![],
            parser: FilterParser::new(filter),
        }
    }
}

impl<'a, I> Iterator for ParserState<'a, I>
    where I: DoubleEndedIterator<Item = &'a Line>
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.parser.kind {
            FilterKind::Empty => self.handle_empty(),
            FilterKind::Content => self.handle_content(),
            _ => self.handle_boundaries(),
        }
    }
}
