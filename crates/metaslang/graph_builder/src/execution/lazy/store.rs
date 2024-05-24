// -*- coding: utf-8 -*-
// ------------------------------------------------------------------------------------------------
// Copyright © 2022, tree-sitter authors.
// Licensed under either of Apache License, Version 2.0, or MIT license, at your option.
// Please see the LICENSE-APACHE or LICENSE-MIT files in this distribution for license details.
// ------------------------------------------------------------------------------------------------

//! Defines store and thunks for lazy DSL evaluation

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use log::trace;
use metaslang_cst::KindTypes;

use super::values::*;
use super::EvaluationContext;
use crate::execution::error::{
    Context, ExecutionError, ResultWithExecutionError, StatementContext,
};
use crate::graph::{SyntaxNodeID, SyntaxNodeRef};
use crate::{graph, Identifier};

/// Variable that points to a thunk in the store
#[derive(Clone, Debug)]
pub(super) struct LazyVariable {
    store_location: usize,
}

impl LazyVariable {
    fn new(store_location: usize) -> Self {
        Self { store_location }
    }

    pub(super) fn evaluate<KT: KindTypes>(
        &self,
        exec: &mut EvaluationContext<'_, KT>,
    ) -> Result<graph::Value, ExecutionError> {
        exec.store.evaluate(self, exec)
    }
}

impl fmt::Display for LazyVariable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(load {})", self.store_location)
    }
}

/// Store holding thunks of lazy values
#[derive(Default)]
pub(super) struct LazyStore {
    elements: Vec<Thunk>,
}

impl LazyStore {
    pub(super) fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub(super) fn add(&mut self, value: LazyValue, debug_info: DebugInfo) -> LazyVariable {
        let store_location = self.elements.len();
        let variable = LazyVariable::new(store_location);
        trace!("store {} = {}", store_location, value);
        self.elements.push(Thunk::new(value, debug_info));
        variable
    }

    pub(super) fn evaluate<KT: KindTypes>(
        &self,
        variable: &LazyVariable,
        exec: &mut EvaluationContext<'_, KT>,
    ) -> Result<graph::Value, ExecutionError> {
        let variable = &self.elements[variable.store_location];
        let debug_info = variable.debug_info.clone();
        let value = variable.force(exec).with_context(|| debug_info.0.into())?;
        Ok(value)
    }

    pub(super) fn evaluate_all<KT: KindTypes>(
        &self,
        exec: &mut EvaluationContext<'_, KT>,
    ) -> Result<(), ExecutionError> {
        for variable in &self.elements {
            let debug_info = variable.debug_info.clone();
            variable.force(exec).with_context(|| debug_info.0.into())?;
        }
        Ok(())
    }
}

/// Data structure to hold scoped variables with lazy keys and values
pub(super) struct LazyScopedVariables {
    variables: HashMap<Identifier, Cell<ScopedValues>>,
}

impl LazyScopedVariables {
    pub(super) fn new() -> Self {
        LazyScopedVariables {
            variables: HashMap::new(),
        }
    }

    pub(super) fn add(
        &mut self,
        scope: LazyValue,
        name: Identifier,
        value: LazyValue,
        debug_info: DebugInfo,
    ) -> Result<(), ExecutionError> {
        let values = self
            .variables
            .entry(name.clone())
            .or_insert_with(|| Cell::new(ScopedValues::new()));
        match values.replace(ScopedValues::Forcing) {
            ScopedValues::Unforced(mut pairs) => {
                pairs.push((scope, value, debug_info));
                values.replace(ScopedValues::Unforced(pairs));
                Ok(())
            }
            ScopedValues::Forcing => Err(ExecutionError::RecursivelyDefinedScopedVariable(
                format!("{}", name),
            )),
            ScopedValues::Forced(map) => {
                values.replace(ScopedValues::Forced(map));
                Err(ExecutionError::VariableScopesAlreadyForced(format!(
                    "{}",
                    name
                )))
            }
        }
    }

    pub(super) fn evaluate<KT: KindTypes>(
        &self,
        scope: &SyntaxNodeRef,
        name: &Identifier,
        exec: &mut EvaluationContext<'_, KT>,
    ) -> Result<LazyValue, ExecutionError> {
        let cell = match self.variables.get(name) {
            Some(v) => v,
            None => {
                return Err(ExecutionError::UndefinedScopedVariable(format!(
                    "{}.{}",
                    scope, name,
                )));
            }
        };
        let values = cell.replace(ScopedValues::Forcing);
        let map = self.force(name, values, exec)?;

        let mut result = None;

        if let Some(value) = map.get(&scope.index) {
            result = Some(value.clone());
        } else if exec.inherited_variables.contains(name) {
            if let Some(mut scope) = exec.graph.syntax_nodes.get(&scope.index).cloned() {
                while scope.go_to_parent() {
                    if let Some(value) = map.get(&(scope.node().id() as u32)) {
                        result = Some(value.clone());
                        break;
                    }
                }
            }
        }

        cell.replace(ScopedValues::Forced(map));
        result.ok_or_else(|| ExecutionError::UndefinedScopedVariable(format!("{}.{}", scope, name)))
    }

    pub(super) fn evaluate_all<KT: KindTypes>(
        &self,
        exec: &mut EvaluationContext<'_, KT>,
    ) -> Result<(), ExecutionError> {
        for (name, cell) in &self.variables {
            let values = cell.replace(ScopedValues::Forcing);
            let map = self.force(name, values, exec)?;
            cell.replace(ScopedValues::Forced(map));
        }
        Ok(())
    }

    fn force<KT: KindTypes>(
        &self,
        name: &Identifier,
        values: ScopedValues,
        exec: &mut EvaluationContext<'_, KT>,
    ) -> Result<HashMap<SyntaxNodeID, LazyValue>, ExecutionError> {
        match values {
            ScopedValues::Unforced(pairs) => {
                let mut values = HashMap::new();
                let mut debug_infos = HashMap::new();
                for (scope, value, debug_info) in pairs.into_iter() {
                    let node = scope
                        .evaluate_as_syntax_node(exec)
                        .with_context(|| format!("Evaluating scope of variable _.{}", name,).into())
                        .with_context(|| debug_info.0.clone().into())?;
                    match (
                        values.insert(node.index, value.clone()),
                        debug_infos.insert(node.index, debug_info.clone()),
                    ) {
                        (Some(_), Some(prev_debug_info)) => {
                            return Err(ExecutionError::DuplicateVariable(format!(
                                "{}.{}",
                                node, name,
                            )))
                            .with_context(|| (prev_debug_info.0, debug_info.0).into());
                        }
                        (Some(_), None) => {
                            unreachable!(
                                "previous value for syntax node {} without previous debug info",
                                node
                            )
                        }
                        _ => {}
                    };
                }
                Ok(values)
            }
            ScopedValues::Forcing => Err(ExecutionError::RecursivelyDefinedScopedVariable(
                format!("_.{}", name),
            )),
            ScopedValues::Forced(map) => Ok(map),
        }
    }
}

enum ScopedValues {
    Unforced(Vec<(LazyValue, LazyValue, DebugInfo)>),
    Forcing,
    Forced(HashMap<SyntaxNodeID, LazyValue>),
}

impl ScopedValues {
    fn new() -> ScopedValues {
        ScopedValues::Unforced(Vec::new())
    }
}

/// Thunk holding a lazy value or a forced graph value
struct Thunk {
    state: Rc<RefCell<ThunkState>>,
    debug_info: DebugInfo,
}

enum ThunkState {
    Unforced(LazyValue),
    Forcing,
    Forced(graph::Value),
}

impl fmt::Display for ThunkState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unforced(value) => write!(f, "?{{{}}}", value),
            Self::Forcing => write!(f, "~{{?}}"),
            Self::Forced(value) => write!(f, "!{{{}}}", value),
        }
    }
}

impl Thunk {
    fn new(value: LazyValue, debug_info: DebugInfo) -> Thunk {
        Thunk {
            state: Rc::new(RefCell::new(ThunkState::Unforced(value))),
            debug_info,
        }
    }

    fn force<KT: KindTypes>(
        &self,
        exec: &mut EvaluationContext<'_, KT>,
    ) -> Result<graph::Value, ExecutionError> {
        let state = self.state.replace(ThunkState::Forcing);
        trace!("force {}", state);
        let value = match state {
            ThunkState::Unforced(value) => {
                // it is important that we do not hold a borrow of self.forced_values when executing self.value.evaluate
                let value = value.evaluate(exec)?;
                Ok(value)
            }
            ThunkState::Forced(value) => Ok(value),
            ThunkState::Forcing => Err(ExecutionError::RecursivelyDefinedVariable(format!(
                "{}",
                self.debug_info
            ))),
        }?;
        *self.state.borrow_mut() = ThunkState::Forced(value.clone());
        Ok(value)
    }
}

/// Debug info for tracking origins of values
#[derive(Debug, Clone)]
pub(super) struct DebugInfo(StatementContext);

impl From<StatementContext> for DebugInfo {
    fn from(value: StatementContext) -> Self {
        Self(value)
    }
}

impl From<DebugInfo> for StatementContext {
    fn from(value: DebugInfo) -> Self {
        value.0
    }
}

impl From<DebugInfo> for Context {
    fn from(value: DebugInfo) -> Self {
        value.0.into()
    }
}

impl fmt::Display for DebugInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.statement_location)
    }
}
