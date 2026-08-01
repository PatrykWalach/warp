/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::cmp::Ordering;
use std::fmt;

use common::SourceLocationKey;
use common::Span;
use common::WithLocation;
use intern::string_key::StringKey;

use crate::relay_lexer::TokenKind;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Token {
    pub span: Span,
    pub kind: TokenKind,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Identifier {
    pub token: Token,
    pub value: StringKey,
}

impl fmt::Debug for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Identifier")
            .field("span", &self.span())
            .field("token", &self.token)
            .field("value", &self.value)
            .finish()
    }
}

impl Identifier {
    pub fn span(&self) -> Span {
        self.token.span
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}", self.value))
    }
}

impl Ord for Identifier {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for Identifier {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Identifier {
    pub fn name_with_location(&self, file: SourceLocationKey) -> WithLocation<StringKey> {
        WithLocation::from_span(file, self.span(), self.value)
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct List<T> {
    pub start: Token,
    pub items: Vec<T>,
    pub end: Token,
}

impl<T: fmt::Debug> fmt::Debug for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("List")
            .field("span", &self.span())
            .field("start", &self.start)
            .field("items", &self.items)
            .field("end", &self.end)
            .finish()
    }
}

impl<T> List<T> {
    pub fn span(&self) -> Span {
        Span {
            start: self.start.span.start,
            end: self.end.span.end,
        }
    }
    pub fn generated(items: Vec<T>) -> Self {
        Self {
            start: Token {
                span: Span::empty(),
                kind: TokenKind::OpenBrace,
            },
            items,
            end: Token {
                span: Span::empty(),
                kind: TokenKind::CloseBrace,
            },
        }
    }
}
