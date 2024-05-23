// -*- coding: utf-8 -*-
// ------------------------------------------------------------------------------------------------
// Copyright © 2022, tree-sitter authors.
// Licensed under either of Apache License, Version 2.0, or MIT license, at your option.
// Please see the LICENSE-APACHE or LICENSE-MIT files in this distribution for license details.
// ------------------------------------------------------------------------------------------------

//! Data types and functions for finding and displaying tree-sitter parse errors.

use std::ops::Range;
use std::path::Path;

#[cfg(feature = "term-colors")]
use colored::Colorize;

//-----------------------------------------------------------------------------

/// Excerpts of source from either the target language file or the tsg rules file.
pub struct Excerpt<'a> {
    path: &'a Path,
    source: Option<&'a str>,
    row: usize,
    columns: Range<usize>,
    indent: usize,
}

impl<'a> Excerpt<'a> {
    pub fn from_source(
        path: &'a Path,
        source: &'a str,
        row: usize,
        mut columns: Range<usize>,
        indent: usize,
    ) -> Excerpt<'a> {
        let source = source.lines().nth(row);
        columns.end = std::cmp::min(columns.end, source.map(|s| s.len()).unwrap_or_default());
        Excerpt {
            path,
            source,
            row,
            columns,
            indent,
        }
    }

    fn gutter_width(&self) -> usize {
        ((self.row + 1) as f64).log10() as usize + 1
    }
}

impl<'a> std::fmt::Display for Excerpt<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // path and line/col
        writeln!(
            f,
            "{}{}:{}:{}:",
            " ".repeat(self.indent),
            white_bold(&self.path.to_string_lossy()),
            white_bold(&format!("{}", self.row + 1)),
            white_bold(&format!("{}", self.columns.start + 1)),
        )?;
        if let Some(source) = self.source {
            // first line: line number & source
            writeln!(
                f,
                "{}{}{}{}",
                " ".repeat(self.indent),
                blue(&format!("{}", self.row + 1)),
                blue(" | "),
                source,
            )?;
            // second line: caret
            writeln!(
                f,
                "{}{}{}{}{}",
                " ".repeat(self.indent),
                " ".repeat(self.gutter_width()),
                blue(" | "),
                " ".repeat(self.columns.start),
                green_bold(&"^".repeat(self.columns.len()))
            )?;
        } else {
            writeln!(f, "{}{}", " ".repeat(self.indent), "<missing source>",)?;
        }
        Ok(())
    }
}

// coloring functions

#[cfg(feature = "term-colors")]
fn blue(str: &str) -> impl std::fmt::Display {
    str.blue()
}
#[cfg(not(feature = "term-colors"))]
fn blue<'a>(str: &'a str) -> impl std::fmt::Display + 'a {
    str
}

#[cfg(feature = "term-colors")]
fn green_bold(str: &str) -> impl std::fmt::Display {
    str.green().bold()
}
#[cfg(not(feature = "term-colors"))]
fn green_bold<'a>(str: &'a str) -> impl std::fmt::Display + 'a {
    str
}

#[cfg(feature = "term-colors")]
fn white_bold(str: &str) -> impl std::fmt::Display {
    str.white().bold()
}
#[cfg(not(feature = "term-colors"))]
fn white_bold<'a>(str: &'a str) -> impl std::fmt::Display + 'a {
    str
}
