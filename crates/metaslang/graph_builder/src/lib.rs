// -*- coding: utf-8 -*-
// ------------------------------------------------------------------------------------------------
// Copyright © 2021, tree-sitter authors.
// Licensed under either of Apache License, Version 2.0, or MIT license, at your option.
// Please see the LICENSE-APACHE or LICENSE-MIT files in this distribution for license details.
// ------------------------------------------------------------------------------------------------

//! This library defines a [DSL][] for constructing arbitrary graph structures from source code
//! that has been parsed using [tree-sitter][].
//!
//! [DSL]: reference/index.html
//! [tree-sitter]: https://docs.rs/tree-sitter/
//! [tree-sitter-python]: https://docs.rs/tree-sitter-python/
//!
//! # Overview
//!
//! You can use [tree-sitter][] to parse the content of source code into a _concrete syntax tree_.
//! There are many interesting use cases where you want to use this parsed syntax tree to create
//! some _other_ graph structure.  This library lets you do that, using a declarative [DSL][] to
//! identify patterns in the parsed syntax tree, along with rules for which nodes and edges to
//! create for the syntax nodes that match those patterns.  You can also annotate each node and
//! edge with an arbitrary set of attributes.
//!
//! There are no limitations on what graph structure you create: you are not limited to creating a
//! tree, and in particular, you are not limited to creating a tree that "lines" up with the parsed
//! syntax tree.

#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::collapsible_else_if)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::equatable_if_let)]
#![allow(clippy::from_over_into)]
#![allow(clippy::from_str_radix_10)]
#![allow(clippy::if_not_else)]
#![allow(clippy::ignored_unit_patterns)]
#![allow(clippy::into_iter_on_ref)]
#![allow(clippy::iter_without_into_iter)]
#![allow(clippy::manual_let_else)]
#![allow(clippy::map_clone)]
#![allow(clippy::map_unwrap_or)]
#![allow(clippy::match_like_matches_macro)]
#![allow(clippy::needless_bool_assign)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::needless_return)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::needless_pub_self)]
#![allow(clippy::new_without_default)]
#![allow(clippy::range_plus_one)]
#![allow(clippy::redundant_else)]
#![allow(clippy::redundant_pattern_matching)]
#![allow(clippy::return_self_not_must_use)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::struct_field_names)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_lazy_evaluations)]
#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::unused_self)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::writeln_empty_string)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::map_flatten)]
#![allow(clippy::useless_conversion)]
#![allow(clippy::useless_format)]
#![allow(clippy::redundant_field_names)]
#![allow(clippy::write_literal)]
#![allow(clippy::println_empty_string)]
#![allow(clippy::write_with_newline)]
#![allow(clippy::explicit_into_iter_loop)]
#![allow(clippy::unnecessary_sort_by)]
#![allow(clippy::unnecessary_mut_passed)]
#![allow(clippy::manual_string_new)]
#![allow(clippy::explicit_iter_loop)]
#![allow(clippy::single_char_pattern)]
#![allow(clippy::unused_unit)]

#[cfg(test)]
use {env_logger as _, indoc as _, string_interner as _, strum as _, strum_macros as _};

#[cfg(doc)]
pub mod reference;

pub mod ast;
mod checker;
pub mod excerpt;
mod execution;
pub mod functions;
pub mod graph;
// pub mod parse_error;
mod parser;
mod variables;

use std::borrow::Borrow;
use std::hash::Hash;
use std::ops::Deref;
use std::sync::Arc;

pub use execution::error::ExecutionError;
pub use execution::{CancellationError, CancellationFlag, ExecutionConfig, Match, NoCancellation};
pub use parser::{Location, ParseError};
use serde::{Serialize, Serializer};
pub use variables::{Globals as Variables, Iter as VariableIter, VariableError};

/// An identifier that appears in a graph DSL file or in the graph that is produced as an output.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Identifier(Arc<String>);

impl Identifier {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn into_string(mut self) -> String {
        Arc::make_mut(&mut self.0);
        Arc::try_unwrap(self.0).unwrap()
    }
}

impl Borrow<str> for Identifier {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl Deref for Identifier {
    type Target = str;
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<&str> for Identifier {
    fn from(value: &str) -> Identifier {
        Identifier(Arc::new(String::from(value)))
    }
}

impl Hash for Identifier {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl PartialEq<str> for Identifier {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl<'a> PartialEq<&'a str> for Identifier {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_str() == *other
    }
}

impl Serialize for Identifier {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
