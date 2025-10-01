// -*- coding: utf-8 -*-
// ------------------------------------------------------------------------------------------------
// Copyright © 2021, tree-sitter authors.
// Licensed under either of Apache License, Version 2.0, or MIT license, at your option.
// Please see the LICENSE-APACHE or LICENSE-MIT files in this distribution for license details.
// ------------------------------------------------------------------------------------------------

use metaslang_cst::kinds::KindTypes;
use metaslang_cst::query::{CaptureQuantifier, QueryMatch};
use thiserror::Error;

use crate::ast::{CreateEdge, File, Stanza, Variable};
use crate::execution::error::ExecutionError;
use crate::functions::Functions;
use crate::graph::{Attributes, Graph, Value};
use crate::variables::Globals;
use crate::{Identifier, Location};

pub(crate) mod error;
mod lazy;
mod strict;

impl<KT: KindTypes + 'static> File<KT> {
    /// Executes this graph DSL file against a source file.  You must provide the parsed syntax
    /// tree (`tree`) as well as the source text that it was parsed from (`source`).  You also
    /// provide the set of functions and global variables that are available during execution.
    pub fn execute<'tree>(
        &self,
        tree: &'tree metaslang_cst::cursor::Cursor<KT>,
        config: &ExecutionConfig<'_, '_, KT>,
        cancellation_flag: &dyn CancellationFlag,
    ) -> Result<Graph<KT>, ExecutionError> {
        let mut graph = Graph::new();
        self.execute_into(&mut graph, tree, config, cancellation_flag)?;
        Ok(graph)
    }

    /// Executes this graph DSL file against a source file, saving the results into an existing
    /// `Graph` instance.  You must provide the parsed syntax tree (`tree`) as well as the source
    /// text that it was parsed from (`source`).  You also provide the set of functions and global
    /// variables that are available during execution. This variant is useful when you need to
    /// “pre-seed” the graph with some predefined nodes and/or edges before executing the DSL file.
    pub fn execute_into<'tree>(
        &self,
        graph: &mut Graph<KT>,
        tree: &'tree metaslang_cst::cursor::Cursor<KT>,
        config: &ExecutionConfig<'_, '_, KT>,
        cancellation_flag: &dyn CancellationFlag,
    ) -> Result<(), ExecutionError> {
        if config.lazy {
            self.execute_lazy_into(graph, tree, config, cancellation_flag)
        } else {
            self.execute_strict_into(graph, tree, config, cancellation_flag)
        }
    }

    pub(self) fn check_globals(&self, globals: &mut Globals<'_>) -> Result<(), ExecutionError> {
        for global in &self.globals {
            match globals.get(&global.name) {
                None => {
                    if let Some(default) = &global.default {
                        globals
                            .add(global.name.clone(), default.to_string().into())
                            .map_err(|_| {
                                ExecutionError::DuplicateVariable(format!(
                                    "global variable {} already defined",
                                    global.name
                                ))
                            })?;
                    } else {
                        return Err(ExecutionError::MissingGlobalVariable(
                            global.name.as_str().to_string(),
                        ));
                    }
                }
                Some(value) => {
                    if global.quantifier == CaptureQuantifier::ZeroOrMore
                        || global.quantifier == CaptureQuantifier::OneOrMore
                    {
                        if value.as_list().is_err() {
                            return Err(ExecutionError::ExpectedList(
                                global.name.as_str().to_string(),
                            ));
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub fn try_visit_matches<'tree, E, F>(
        &self,
        tree: &'tree metaslang_cst::cursor::Cursor<KT>,
        lazy: bool,
        mut visit: F,
    ) -> Result<(), E>
    where
        F: FnMut(Match<KT>) -> Result<(), E>,
    {
        if lazy {
            self.try_visit_matches_lazy(tree, |stanza, mat| {
                visit(Match {
                    mat,
                    query_location: stanza.range.start,
                })
            })
        } else {
            self.try_visit_matches_strict(tree, |stanza, mat| {
                visit(Match {
                    mat,
                    query_location: stanza.range.start,
                })
            })
        }
    }
}

impl<KT: KindTypes + 'static> Stanza<KT> {
    pub fn try_visit_matches<'tree, E, F>(
        &self,
        tree: &'tree metaslang_cst::cursor::Cursor<KT>,
        mut visit: F,
    ) -> Result<(), E>
    where
        F: FnMut(Match<KT>) -> Result<(), E>,
    {
        self.try_visit_matches_strict(tree, |mat| {
            visit(Match {
                mat,
                query_location: self.range.start,
            })
        })
    }
}

pub struct Match<KT: KindTypes> {
    mat: QueryMatch<KT>,
    query_location: Location,
}

impl<KT: KindTypes> Match<KT> {
    /// Return the top-level matched node.
    pub fn full_capture(&self) -> metaslang_cst::cursor::Cursor<KT> {
        self.mat.root_cursor().clone()
    }

    /// Return the matched nodes for a named capture.
    pub fn named_captures(
        &self,
    ) -> impl Iterator<
        Item = (
            &String,
            CaptureQuantifier,
            impl Iterator<Item = metaslang_cst::cursor::Cursor<KT>>,
        ),
    > {
        self.mat.captures()
    }

    /// Return the matched nodes for a named capture.
    pub fn named_capture(
        &self,
        name: &str,
    ) -> Option<(
        CaptureQuantifier,
        impl Iterator<Item = metaslang_cst::cursor::Cursor<KT>>,
    )> {
        self.mat.capture(name)
    }

    /// Return an iterator over all capture names.
    pub fn capture_names(&self) -> impl Iterator<Item = &String> {
        self.mat.capture_names()
    }

    /// Return the query location.
    pub fn query_location(&self) -> &Location {
        &self.query_location
    }
}

/// Configuration for the execution of a File
pub struct ExecutionConfig<'a, 'g, KT: KindTypes> {
    pub(crate) functions: &'a Functions<KT>,
    pub(crate) globals: &'a Globals<'g>,
    pub(crate) lazy: bool,
    pub(crate) location_attr: Option<Identifier>,
    pub(crate) variable_name_attr: Option<Identifier>,
    pub(crate) match_node_attr: Option<Identifier>,
}

impl<'a, 'g, KT: KindTypes> ExecutionConfig<'a, 'g, KT> {
    pub fn new(functions: &'a Functions<KT>, globals: &'a Globals<'g>) -> Self {
        Self {
            functions,
            globals,
            lazy: false,
            location_attr: None,
            variable_name_attr: None,
            match_node_attr: None,
        }
    }

    pub fn debug_attributes(
        self,
        location_attr: Identifier,
        variable_name_attr: Identifier,
        match_node_attr: Identifier,
    ) -> Self {
        Self {
            functions: self.functions,
            globals: self.globals,
            lazy: self.lazy,
            location_attr: location_attr.into(),
            variable_name_attr: variable_name_attr.into(),
            match_node_attr: match_node_attr.into(),
        }
    }

    pub fn lazy(self, lazy: bool) -> Self {
        Self {
            functions: self.functions,
            globals: self.globals,
            lazy,
            location_attr: self.location_attr,
            variable_name_attr: self.variable_name_attr,
            match_node_attr: self.match_node_attr,
        }
    }
}

/// Trait to signal that the execution is cancelled
pub trait CancellationFlag {
    fn check(&self, at: &'static str) -> Result<(), CancellationError>;
}

pub struct NoCancellation;
impl CancellationFlag for NoCancellation {
    fn check(&self, _at: &'static str) -> Result<(), CancellationError> {
        Ok(())
    }
}

#[derive(Debug, Error)]
#[error("Cancelled at \"{0}\"")]
pub struct CancellationError(pub &'static str);

impl Value {
    pub fn from_nodes<KT: KindTypes, CI: IntoIterator<Item = metaslang_cst::cursor::Cursor<KT>>>(
        graph: &mut Graph<KT>,
        cursors: CI,
        quantifier: CaptureQuantifier,
    ) -> Value {
        let mut cursors = cursors.into_iter();
        match quantifier {
            CaptureQuantifier::One => {
                let syntax_node = graph.add_syntax_node(cursors.next().expect("missing capture"));
                syntax_node.into()
            }
            CaptureQuantifier::ZeroOrMore | CaptureQuantifier::OneOrMore => {
                let syntax_nodes = cursors
                    .map(|c| graph.add_syntax_node(c).into())
                    .collect::<Vec<Value>>();
                syntax_nodes.into()
            }
            CaptureQuantifier::ZeroOrOne => match cursors.next() {
                None => Value::Null.into(),
                Some(cursor) => {
                    let syntax_node = graph.add_syntax_node(cursor);
                    syntax_node.into()
                }
            },
        }
    }
}

impl CreateEdge {
    pub(crate) fn add_debug_attrs<KT: KindTypes>(
        &self,
        attributes: &mut Attributes,
        config: &ExecutionConfig<'_, '_, KT>,
    ) -> Result<(), ExecutionError> {
        if let Some(location_attr) = &config.location_attr {
            attributes
                .add(
                    location_attr.clone(),
                    format!(
                        "line {} column {}",
                        self.location.row + 1,
                        self.location.column + 1
                    ),
                )
                .map_err(|_| ExecutionError::DuplicateAttribute(location_attr.as_str().into()))?;
        }
        Ok(())
    }
}
impl Variable {
    pub(crate) fn add_debug_attrs<KT: KindTypes>(
        &self,
        attributes: &mut Attributes,
        config: &ExecutionConfig<'_, '_, KT>,
    ) -> Result<(), ExecutionError> {
        if let Some(variable_name_attr) = &config.variable_name_attr {
            attributes
                .add(variable_name_attr.clone(), format!("{}", self))
                .map_err(|_| {
                    ExecutionError::DuplicateAttribute(variable_name_attr.as_str().into())
                })?;
        }
        if let Some(location_attr) = &config.location_attr {
            let location = match &self {
                Variable::Scoped(v) => v.location,
                Variable::Unscoped(v) => v.location,
            };
            attributes
                .add(
                    location_attr.clone(),
                    format!("line {} column {}", location.row + 1, location.column + 1),
                )
                .map_err(|_| ExecutionError::DuplicateAttribute(location_attr.as_str().into()))?;
        }
        Ok(())
    }
}
