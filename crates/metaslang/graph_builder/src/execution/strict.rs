// -*- coding: utf-8 -*-
// ------------------------------------------------------------------------------------------------
// Copyright © 2021, tree-sitter authors.
// Licensed under either of Apache License, Version 2.0, or MIT license, at your option.
// Please see the LICENSE-APACHE or LICENSE-MIT files in this distribution for license details.
// ------------------------------------------------------------------------------------------------

use std::collections::{BTreeSet, HashMap, HashSet};

use metaslang_cst::kinds::KindTypes;
use metaslang_cst::query::QueryMatch;

// use tree_sitter::{QueryCursor, QueryMatch, Tree};
use crate::ast::{
    AddEdgeAttribute, AddGraphNodeAttribute, Assign, Attribute, AttributeShorthand,
    AttributeShorthands, Call, Capture, Condition, CreateEdge, CreateGraphNode, DeclareImmutable,
    DeclareMutable, Expression, File, ForIn, If, IntegerConstant, ListComprehension, ListLiteral,
    Print, RegexCapture, Scan, ScopedVariable, SetComprehension, SetLiteral, Stanza, Statement,
    StringConstant, UnscopedVariable, Variable,
};
use crate::execution::error::{ExecutionError, ResultWithExecutionError, StatementContext};
use crate::execution::{CancellationFlag, ExecutionConfig};
use crate::graph::{Graph, SyntaxNodeID, SyntaxNodeRef, Value};
use crate::variables::{Globals, MutVariables, VariableMap, Variables};
use crate::{Identifier, Location};

impl<KT: KindTypes + 'static> File<KT> {
    /// Executes this graph DSL file against a source file, saving the results into an existing
    /// `Graph` instance.  You must provide the parsed syntax tree (`tree`) as well as the source
    /// text that it was parsed from (`source`).  You also provide the set of functions and global
    /// variables that are available during execution. This variant is useful when you need to
    /// “pre-seed” the graph with some predefined nodes and/or edges before executing the DSL file.
    pub(super) fn execute_strict_into<'tree>(
        &self,
        graph: &mut Graph<KT>,
        tree: &'tree metaslang_cst::cursor::Cursor<KT>,
        config: &ExecutionConfig<'_, '_, KT>,
        cancellation_flag: &dyn CancellationFlag,
    ) -> Result<(), ExecutionError> {
        let mut globals = Globals::nested(config.globals);
        self.check_globals(&mut globals)?;
        let mut config = ExecutionConfig {
            functions: config.functions,
            globals: &globals,
            lazy: config.lazy,
            location_attr: config.location_attr.clone(),
            variable_name_attr: config.variable_name_attr.clone(),
            match_node_attr: config.match_node_attr.clone(),
        };

        let mut locals = VariableMap::new();
        let mut scoped = ScopedVariables::new();
        let current_regex_captures = Vec::new();
        let mut function_parameters = Vec::new();

        self.try_visit_matches_strict(tree, |stanza, mat| {
            stanza.execute(
                &mat,
                graph,
                &mut config,
                &mut locals,
                &mut scoped,
                &current_regex_captures,
                &mut function_parameters,
                &self.inherited_variables,
                &self.shorthands,
                cancellation_flag,
            )
        })?;

        Ok(())
    }

    pub(super) fn try_visit_matches_strict<'tree, E, F>(
        &self,
        tree: &'tree metaslang_cst::cursor::Cursor<KT>,
        mut visit: F,
    ) -> Result<(), E>
    where
        F: FnMut(&Stanza<KT>, QueryMatch<KT>) -> Result<(), E>,
    {
        for stanza in &self.stanzas {
            stanza.try_visit_matches_strict(tree, |mat| visit(stanza, mat))?;
        }
        Ok(())
    }
}

/// State that is threaded through the execution
struct ExecutionContext<'a, 'c, 'g, 's, KT: KindTypes> {
    graph: &'a mut Graph<KT>,
    config: &'a ExecutionConfig<'c, 'g, KT>,
    locals: &'a mut dyn MutVariables<Value>,
    scoped: &'a mut ScopedVariables<'s>,
    current_regex_captures: &'a Vec<String>,
    function_parameters: &'a mut Vec<Value>,
    mat: &'a QueryMatch<KT>,
    error_context: StatementContext,
    inherited_variables: &'a HashSet<Identifier>,
    shorthands: &'a AttributeShorthands,
    cancellation_flag: &'a dyn CancellationFlag,
}

struct ScopedVariables<'a> {
    scopes: HashMap<SyntaxNodeID, VariableMap<'a, Value>>,
}

impl<'a> ScopedVariables<'a> {
    fn new() -> Self {
        Self {
            scopes: HashMap::new(),
        }
    }

    fn get_mut(&mut self, scope: SyntaxNodeRef) -> &mut VariableMap<'a, Value> {
        self.scopes.entry(scope.index).or_insert(VariableMap::new())
    }

    fn try_get(&self, index: SyntaxNodeID) -> Option<&VariableMap<'a, Value>> {
        self.scopes.get(&index)
    }
}

impl<KT: KindTypes + 'static> Stanza<KT> {
    fn execute<'g, 'l, 's>(
        &self,
        mat: &QueryMatch<KT>,
        graph: &mut Graph<KT>,
        config: &ExecutionConfig<'_, 'g, KT>,
        locals: &mut VariableMap<'l, Value>,
        scoped: &mut ScopedVariables<'s>,
        current_regex_captures: &Vec<String>,
        function_parameters: &mut Vec<Value>,
        inherited_variables: &HashSet<Identifier>,
        shorthands: &AttributeShorthands,
        cancellation_flag: &dyn CancellationFlag,
    ) -> Result<(), ExecutionError> {
        locals.clear();
        for statement in &self.statements {
            let error_context = {
                let cursor = &mat.root_cursor();
                StatementContext::new(&statement, &self, cursor)
            };
            let mut exec = ExecutionContext {
                graph,
                config,
                locals,
                scoped,
                current_regex_captures,
                function_parameters,
                mat: &mat,
                error_context,
                inherited_variables,
                shorthands,
                cancellation_flag,
            };
            statement
                .execute(&mut exec)
                .with_context(|| exec.error_context.into())?;
        }
        Ok(())
    }

    pub(super) fn try_visit_matches_strict<'tree, E, F>(
        &self,
        tree: &'tree metaslang_cst::cursor::Cursor<KT>,
        mut visit: F,
    ) -> Result<(), E>
    where
        F: FnMut(QueryMatch<KT>) -> Result<(), E>,
    {
        let matches = tree.clone().query(vec![self.query.clone()]);
        for mat in matches {
            visit(mat)?;
        }
        Ok(())
    }
}

impl Statement {
    pub fn location(&self) -> Location {
        match self {
            Statement::DeclareImmutable(s) => s.location,
            Statement::DeclareMutable(s) => s.location,
            Statement::Assign(s) => s.location,
            Statement::CreateGraphNode(s) => s.location,
            Statement::AddGraphNodeAttribute(s) => s.location,
            Statement::CreateEdge(s) => s.location,
            Statement::AddEdgeAttribute(s) => s.location,
            Statement::Scan(s) => s.location,
            Statement::Print(s) => s.location,
            Statement::If(s) => s.location,
            Statement::ForIn(s) => s.location,
        }
    }

    fn execute<KT: KindTypes>(
        &self,
        exec: &mut ExecutionContext<'_, '_, '_, '_, KT>,
    ) -> Result<(), ExecutionError> {
        exec.cancellation_flag.check("executing statement")?;
        match self {
            Statement::DeclareImmutable(statement) => statement.execute(exec),
            Statement::DeclareMutable(statement) => statement.execute(exec),
            Statement::Assign(statement) => statement.execute(exec),
            Statement::CreateGraphNode(statement) => statement.execute(exec),
            Statement::AddGraphNodeAttribute(statement) => statement.execute(exec),
            Statement::CreateEdge(statement) => statement.execute(exec),
            Statement::AddEdgeAttribute(statement) => statement.execute(exec),
            Statement::Scan(statement) => statement.execute(exec),
            Statement::Print(statement) => statement.execute(exec),
            Statement::If(statement) => statement.execute(exec),
            Statement::ForIn(statement) => statement.execute(exec),
        }
    }
}

impl DeclareImmutable {
    fn execute<KT: KindTypes>(
        &self,
        exec: &mut ExecutionContext<'_, '_, '_, '_, KT>,
    ) -> Result<(), ExecutionError> {
        let value = self.value.evaluate(exec)?;
        self.variable.add(exec, value, false)
    }
}

impl DeclareMutable {
    fn execute<KT: KindTypes>(
        &self,
        exec: &mut ExecutionContext<'_, '_, '_, '_, KT>,
    ) -> Result<(), ExecutionError> {
        let value = self.value.evaluate(exec)?;
        self.variable.add(exec, value, true)
    }
}

impl Assign {
    fn execute<KT: KindTypes>(
        &self,
        exec: &mut ExecutionContext<'_, '_, '_, '_, KT>,
    ) -> Result<(), ExecutionError> {
        let value = self.value.evaluate(exec)?;
        self.variable.set(exec, value)
    }
}

impl CreateGraphNode {
    fn execute<KT: KindTypes>(
        &self,
        exec: &mut ExecutionContext<'_, '_, '_, '_, KT>,
    ) -> Result<(), ExecutionError> {
        let graph_node = exec.graph.add_graph_node();
        self.node
            .add_debug_attrs(&mut exec.graph[graph_node].attributes, exec.config)?;
        if let Some(match_node_attr) = &exec.config.match_node_attr {
            let cursor = exec.mat.root_cursor().clone();
            let syn_node = exec.graph.add_syntax_node(cursor);
            exec.graph[graph_node]
                .attributes
                .add(match_node_attr.clone(), syn_node)
                .map_err(|_| {
                    ExecutionError::DuplicateAttribute(format!(
                        " {} on graph node ({}) in {}",
                        match_node_attr, graph_node, self,
                    ))
                })?;
        }
        let value = Value::GraphNode(graph_node);
        self.node.add(exec, value, false)
    }
}

impl AddGraphNodeAttribute {
    fn execute<KT: KindTypes>(
        &self,
        exec: &mut ExecutionContext<'_, '_, '_, '_, KT>,
    ) -> Result<(), ExecutionError> {
        let node = self.node.evaluate(exec)?.into_graph_node_ref()?;
        let add_attribute =
            |exec: &mut ExecutionContext<'_, '_, '_, '_, KT>, name: Identifier, value: Value| {
                exec.graph[node]
                    .attributes
                    .add(name.clone(), value)
                    .map_err(|_| {
                        ExecutionError::DuplicateAttribute(format!(
                            " {} on graph node ({}) in {}",
                            name, node, self,
                        ))
                    })
            };
        for attribute in &self.attributes {
            attribute.execute(exec, &add_attribute)?;
        }
        Ok(())
    }
}

impl CreateEdge {
    fn execute<KT: KindTypes>(
        &self,
        exec: &mut ExecutionContext<'_, '_, '_, '_, KT>,
    ) -> Result<(), ExecutionError> {
        let source = self.source.evaluate(exec)?.into_graph_node_ref()?;
        let sink = self.sink.evaluate(exec)?.into_graph_node_ref()?;
        let edge = match exec.graph[source].add_edge(sink) {
            Ok(edge) | Err(edge) => edge,
        };
        self.add_debug_attrs(&mut edge.attributes, exec.config)?;
        Ok(())
    }
}

impl AddEdgeAttribute {
    fn execute<KT: KindTypes>(
        &self,
        exec: &mut ExecutionContext<'_, '_, '_, '_, KT>,
    ) -> Result<(), ExecutionError> {
        let source = self.source.evaluate(exec)?.into_graph_node_ref()?;
        let sink = self.sink.evaluate(exec)?.into_graph_node_ref()?;
        let add_attribute =
            |exec: &mut ExecutionContext<'_, '_, '_, '_, KT>, name: Identifier, value: Value| {
                let edge = match exec.graph[source].get_edge_mut(sink) {
                    Some(edge) => Ok(edge),
                    None => Err(ExecutionError::UndefinedEdge(format!(
                        "({} -> {}) in {}",
                        source, sink, self,
                    ))),
                }?;
                edge.attributes.add(name.clone(), value).map_err(|_| {
                    ExecutionError::DuplicateAttribute(format!(
                        " {} on edge ({} -> {}) in {}",
                        name, source, sink, self,
                    ))
                })
            };
        for attribute in &self.attributes {
            attribute.execute(exec, &add_attribute)?;
        }
        Ok(())
    }
}

impl Scan {
    fn execute<KT: KindTypes>(
        &self,
        exec: &mut ExecutionContext<'_, '_, '_, '_, KT>,
    ) -> Result<(), ExecutionError> {
        let match_string = self.value.evaluate(exec)?.into_string()?;

        let mut i = 0;
        let mut matches = Vec::new();
        while i < match_string.len() {
            exec.cancellation_flag.check("processing scan matches")?;
            matches.clear();
            for (index, arm) in self.arms.iter().enumerate() {
                let captures = arm.regex.captures(&match_string[i..]);
                if let Some(captures) = captures {
                    if captures
                        .get(0)
                        .expect("missing regex capture")
                        .range()
                        .is_empty()
                    {
                        return Err(ExecutionError::EmptyRegexCapture(format!(
                            "for regular expression /{}/",
                            arm.regex
                        )));
                    }
                    matches.push((captures, index));
                }
            }

            if matches.is_empty() {
                return Ok(());
            }

            matches.sort_by_key(|(captures, index)| {
                let range = captures.get(0).expect("missing regex capture").range();
                (range.start, *index)
            });

            let (regex_captures, block_index) = &matches[0];
            let arm = &self.arms[*block_index];

            let mut current_regex_captures = Vec::new();
            for regex_capture in regex_captures.iter() {
                current_regex_captures
                    .push(regex_capture.map(|m| m.as_str()).unwrap_or("").to_string());
            }

            let mut arm_locals = VariableMap::nested(exec.locals);
            let mut arm_exec = ExecutionContext {
                graph: exec.graph,
                config: exec.config,
                locals: &mut arm_locals,
                scoped: exec.scoped,
                current_regex_captures: &current_regex_captures,
                function_parameters: exec.function_parameters,
                mat: exec.mat,
                error_context: exec.error_context.clone(),
                inherited_variables: exec.inherited_variables,
                shorthands: exec.shorthands,
                cancellation_flag: exec.cancellation_flag,
            };

            for statement in &arm.statements {
                arm_exec.error_context.update_statement(statement);
                statement
                    .execute(&mut arm_exec)
                    .with_context(|| {
                        format!("matching {} with arm \"{}\"", match_string, arm.regex,).into()
                    })
                    .with_context(|| arm_exec.error_context.clone().into())?;
            }

            i += regex_captures
                .get(0)
                .expect("missing regex capture")
                .range()
                .end;
        }

        Ok(())
    }
}

impl Print {
    fn execute<KT: KindTypes>(
        &self,
        exec: &mut ExecutionContext<'_, '_, '_, '_, KT>,
    ) -> Result<(), ExecutionError> {
        for value in &self.values {
            if let Expression::StringConstant(expr) = value {
                eprint!("{}", expr.value);
            } else {
                let value = value.evaluate(exec)?;
                eprint!("{:?}", value);
            }
        }
        eprintln!();
        Ok(())
    }
}

impl If {
    fn execute<KT: KindTypes>(
        &self,
        exec: &mut ExecutionContext<'_, '_, '_, '_, KT>,
    ) -> Result<(), ExecutionError> {
        for arm in &self.arms {
            let mut result = true;
            for condition in &arm.conditions {
                result &= condition.test(exec)?;
            }
            if result {
                let mut arm_locals = VariableMap::nested(exec.locals);
                let mut arm_exec = ExecutionContext {
                    graph: exec.graph,
                    config: exec.config,
                    locals: &mut arm_locals,
                    scoped: exec.scoped,
                    current_regex_captures: exec.current_regex_captures,
                    function_parameters: exec.function_parameters,
                    mat: exec.mat,
                    error_context: exec.error_context.clone(),
                    inherited_variables: exec.inherited_variables,
                    shorthands: exec.shorthands,
                    cancellation_flag: exec.cancellation_flag,
                };
                for stmt in &arm.statements {
                    arm_exec.error_context.update_statement(stmt);
                    stmt.execute(&mut arm_exec)
                        .with_context(|| arm_exec.error_context.clone().into())?;
                }
                break;
            }
        }
        Ok(())
    }
}

impl Condition {
    fn test<KT: KindTypes>(
        &self,
        exec: &mut ExecutionContext<'_, '_, '_, '_, KT>,
    ) -> Result<bool, ExecutionError> {
        match self {
            Condition::Some { value, .. } => Ok(!value.evaluate(exec)?.is_null()),
            Condition::None { value, .. } => Ok(value.evaluate(exec)?.is_null()),
            Condition::Bool { value, .. } => Ok(value.evaluate(exec)?.into_boolean()?),
        }
    }
}

impl ForIn {
    fn execute<KT: KindTypes>(
        &self,
        exec: &mut ExecutionContext<'_, '_, '_, '_, KT>,
    ) -> Result<(), ExecutionError> {
        let values = self.value.evaluate(exec)?.into_list()?;
        let mut loop_locals = VariableMap::nested(exec.locals);
        for value in values {
            loop_locals.clear();
            let mut loop_exec = ExecutionContext {
                graph: exec.graph,
                config: exec.config,
                locals: &mut loop_locals,
                scoped: exec.scoped,
                current_regex_captures: exec.current_regex_captures,
                function_parameters: exec.function_parameters,
                mat: exec.mat,
                error_context: exec.error_context.clone(),
                inherited_variables: exec.inherited_variables,
                shorthands: exec.shorthands,
                cancellation_flag: exec.cancellation_flag,
            };
            self.variable.add(&mut loop_exec, value, false)?;
            for stmt in &self.statements {
                loop_exec.error_context.update_statement(stmt);
                stmt.execute(&mut loop_exec)
                    .with_context(|| loop_exec.error_context.clone().into())?;
            }
        }
        Ok(())
    }
}

impl Expression {
    fn evaluate<KT: KindTypes>(
        &self,
        exec: &mut ExecutionContext<'_, '_, '_, '_, KT>,
    ) -> Result<Value, ExecutionError> {
        match self {
            Expression::FalseLiteral => Ok(Value::Boolean(false)),
            Expression::NullLiteral => Ok(Value::Null),
            Expression::TrueLiteral => Ok(Value::Boolean(true)),
            Expression::IntegerConstant(expr) => expr.evaluate(exec),
            Expression::StringConstant(expr) => expr.evaluate(exec),
            Expression::ListLiteral(expr) => expr.evaluate(exec),
            Expression::SetLiteral(expr) => expr.evaluate(exec),
            Expression::ListComprehension(expr) => expr.evaluate(exec),
            Expression::SetComprehension(expr) => expr.evaluate(exec),
            Expression::Capture(expr) => expr.evaluate(exec),
            Expression::Variable(expr) => expr.evaluate(exec),
            Expression::Call(expr) => expr.evaluate(exec),
            Expression::RegexCapture(expr) => expr.evaluate(exec),
        }
    }
}

impl IntegerConstant {
    fn evaluate<KT: KindTypes>(
        &self,
        _exec: &mut ExecutionContext<'_, '_, '_, '_, KT>,
    ) -> Result<Value, ExecutionError> {
        Ok(Value::Integer(self.value))
    }
}

impl StringConstant {
    fn evaluate<KT: KindTypes>(
        &self,
        _exec: &mut ExecutionContext<'_, '_, '_, '_, KT>,
    ) -> Result<Value, ExecutionError> {
        Ok(Value::String(self.value.clone()))
    }
}

impl ListLiteral {
    fn evaluate<KT: KindTypes>(
        &self,
        exec: &mut ExecutionContext<'_, '_, '_, '_, KT>,
    ) -> Result<Value, ExecutionError> {
        let elements = self
            .elements
            .iter()
            .map(|e| e.evaluate(exec))
            .collect::<Result<_, _>>()?;
        Ok(Value::List(elements))
    }
}

impl ListComprehension {
    fn evaluate<KT: KindTypes>(
        &self,
        exec: &mut ExecutionContext<'_, '_, '_, '_, KT>,
    ) -> Result<Value, ExecutionError> {
        let values = self.value.evaluate(exec)?.into_list()?;
        let mut elements = Vec::new();
        let mut loop_locals = VariableMap::nested(exec.locals);
        for value in values {
            loop_locals.clear();
            let mut loop_exec = ExecutionContext {
                graph: exec.graph,
                config: exec.config,
                locals: &mut loop_locals,
                scoped: exec.scoped,
                current_regex_captures: exec.current_regex_captures,
                function_parameters: exec.function_parameters,
                mat: exec.mat,
                error_context: exec.error_context.clone(),
                inherited_variables: exec.inherited_variables,
                shorthands: exec.shorthands,
                cancellation_flag: exec.cancellation_flag,
            };
            self.variable.add(&mut loop_exec, value, false)?;
            let element = self.element.evaluate(&mut loop_exec)?;
            elements.push(element);
        }
        Ok(Value::List(elements))
    }
}

impl SetLiteral {
    fn evaluate<KT: KindTypes>(
        &self,
        exec: &mut ExecutionContext<'_, '_, '_, '_, KT>,
    ) -> Result<Value, ExecutionError> {
        let elements = self
            .elements
            .iter()
            .map(|e| e.evaluate(exec))
            .collect::<Result<_, _>>()?;
        Ok(Value::Set(elements))
    }
}

impl SetComprehension {
    fn evaluate<KT: KindTypes>(
        &self,
        exec: &mut ExecutionContext<'_, '_, '_, '_, KT>,
    ) -> Result<Value, ExecutionError> {
        let values = self.value.evaluate(exec)?.into_list()?;
        let mut elements = BTreeSet::new();
        let mut loop_locals = VariableMap::nested(exec.locals);
        for value in values {
            loop_locals.clear();
            let mut loop_exec = ExecutionContext {
                graph: exec.graph,
                config: exec.config,
                locals: &mut loop_locals,
                scoped: exec.scoped,
                current_regex_captures: exec.current_regex_captures,
                function_parameters: exec.function_parameters,
                mat: exec.mat,
                error_context: exec.error_context.clone(),
                inherited_variables: exec.inherited_variables,
                shorthands: exec.shorthands,
                cancellation_flag: exec.cancellation_flag,
            };
            self.variable.add(&mut loop_exec, value, false)?;
            let element = self.element.evaluate(&mut loop_exec)?;
            elements.insert(element);
        }
        Ok(Value::Set(elements))
    }
}

impl Capture {
    fn evaluate<KT: KindTypes>(
        &self,
        exec: &mut ExecutionContext<'_, '_, '_, '_, KT>,
    ) -> Result<Value, ExecutionError> {
        let cursors = exec.mat.capture(&self.name.0).unwrap().1;
        Ok(Value::from_nodes(exec.graph, cursors, self.quantifier).into())
    }
}

impl Call {
    fn evaluate<KT: KindTypes>(
        &self,
        exec: &mut ExecutionContext<'_, '_, '_, '_, KT>,
    ) -> Result<Value, ExecutionError> {
        for parameter in &self.parameters {
            let parameter = parameter.evaluate(exec)?;
            exec.function_parameters.push(parameter);
        }
        exec.config.functions.call(
            &self.function,
            exec.graph,
            &mut exec
                .function_parameters
                .drain(exec.function_parameters.len() - self.parameters.len()..),
        )
    }
}

impl RegexCapture {
    fn evaluate<KT: KindTypes>(
        &self,
        exec: &mut ExecutionContext<'_, '_, '_, '_, KT>,
    ) -> Result<Value, ExecutionError> {
        let capture = exec
            .current_regex_captures
            .get(self.match_index)
            .ok_or(ExecutionError::UndefinedRegexCapture(format!("{}", self)))?;
        Ok(Value::String(capture.clone()))
    }
}

impl Variable {
    fn evaluate<KT: KindTypes>(
        &self,
        exec: &mut ExecutionContext<'_, '_, '_, '_, KT>,
    ) -> Result<Value, ExecutionError> {
        let value = self.get(exec)?;
        Ok(value.clone())
    }
}

impl Variable {
    fn get<'a, KT: KindTypes>(
        &self,
        exec: &'a mut ExecutionContext<'_, '_, '_, '_, KT>,
    ) -> Result<&'a Value, ExecutionError> {
        match self {
            Variable::Scoped(variable) => variable.get(exec),
            Variable::Unscoped(variable) => variable.get(exec),
        }
    }

    fn add<KT: KindTypes>(
        &self,
        exec: &mut ExecutionContext<'_, '_, '_, '_, KT>,
        value: Value,
        mutable: bool,
    ) -> Result<(), ExecutionError> {
        match self {
            Variable::Scoped(variable) => variable.add(exec, value, mutable),
            Variable::Unscoped(variable) => variable.add(exec, value, mutable),
        }
    }

    fn set<KT: KindTypes>(
        &self,
        exec: &mut ExecutionContext<'_, '_, '_, '_, KT>,
        value: Value,
    ) -> Result<(), ExecutionError> {
        match self {
            Variable::Scoped(variable) => variable.set(exec, value),
            Variable::Unscoped(variable) => variable.set(exec, value),
        }
    }
}

impl ScopedVariable {
    fn get<'a, KT: KindTypes>(
        &self,
        exec: &'a mut ExecutionContext<'_, '_, '_, '_, KT>,
    ) -> Result<&'a Value, ExecutionError> {
        let scope = self.scope.evaluate(exec)?;
        let scope = match scope {
            Value::SyntaxNode(scope) => scope,
            _ => {
                return Err(ExecutionError::InvalidVariableScope(format!(
                    "got {}",
                    scope
                )))
            }
        };

        // search this node
        if let Some(value) = exec
            .scoped
            .try_get(scope.index)
            .and_then(|v| v.get(&self.name))
        {
            return Ok(value);
        }

        // search parent nodes
        if exec.inherited_variables.contains(&self.name) {
            if let Some(mut cursor) = exec.graph.syntax_nodes.get(&scope.index).cloned() {
                while cursor.go_to_parent() {
                    if let Some(value) = exec
                        .scoped
                        .try_get(cursor.node().id())
                        .and_then(|v| v.get(&self.name))
                    {
                        return Ok(value);
                    }
                }
            }
        }

        Err(ExecutionError::UndefinedVariable(format!(
            "{} on node {}",
            self, scope
        )))
    }

    fn add<KT: KindTypes>(
        &self,
        exec: &mut ExecutionContext<'_, '_, '_, '_, KT>,
        value: Value,
        mutable: bool,
    ) -> Result<(), ExecutionError> {
        let scope = self.scope.evaluate(exec)?;
        let scope = match scope {
            Value::SyntaxNode(scope) => scope,
            _ => {
                return Err(ExecutionError::InvalidVariableScope(format!(
                    "got {}",
                    scope
                )))
            }
        };
        let variables = exec.scoped.get_mut(scope);
        variables
            .add(self.name.clone(), value, mutable)
            .map_err(|_| ExecutionError::DuplicateVariable(format!("{}", self)))
    }

    fn set<KT: KindTypes>(
        &self,
        exec: &mut ExecutionContext<'_, '_, '_, '_, KT>,
        value: Value,
    ) -> Result<(), ExecutionError> {
        let scope = self.scope.evaluate(exec)?;
        let scope = match scope {
            Value::SyntaxNode(scope) => scope,
            _ => {
                return Err(ExecutionError::InvalidVariableScope(format!(
                    "got {}",
                    scope,
                )))
            }
        };
        let variables = exec.scoped.get_mut(scope);
        variables
            .set(self.name.clone(), value)
            .map_err(|_| ExecutionError::DuplicateVariable(format!("{}", self)))
    }
}

impl UnscopedVariable {
    fn get<'a, KT: KindTypes>(
        &self,
        exec: &'a mut ExecutionContext<'_, '_, '_, '_, KT>,
    ) -> Result<&'a Value, ExecutionError> {
        if let Some(value) = exec.config.globals.get(&self.name) {
            Some(value)
        } else {
            exec.locals.get(&self.name)
        }
        .ok_or_else(|| ExecutionError::UndefinedVariable(format!("{}", self)))
    }

    fn add<KT: KindTypes>(
        &self,
        exec: &mut ExecutionContext<'_, '_, '_, '_, KT>,
        value: Value,
        mutable: bool,
    ) -> Result<(), ExecutionError> {
        if exec.config.globals.get(&self.name).is_some() {
            return Err(ExecutionError::DuplicateVariable(format!(
                " global {}",
                self,
            )));
        }
        exec.locals
            .add(self.name.clone(), value, mutable)
            .map_err(|_| ExecutionError::DuplicateVariable(format!(" local {}", self)))
    }

    fn set<KT: KindTypes>(
        &self,
        exec: &mut ExecutionContext<'_, '_, '_, '_, KT>,
        value: Value,
    ) -> Result<(), ExecutionError> {
        if exec.config.globals.get(&self.name).is_some() {
            return Err(ExecutionError::CannotAssignImmutableVariable(format!(
                " global {}",
                self,
            )));
        }
        exec.locals.set(self.name.clone(), value).map_err(|_| {
            if exec.locals.get(&self.name).is_some() {
                ExecutionError::CannotAssignImmutableVariable(format!("{}", self))
            } else {
                ExecutionError::UndefinedVariable(format!("{}", self))
            }
        })
    }
}

impl Attribute {
    fn execute<F, KT: KindTypes>(
        &self,
        exec: &mut ExecutionContext<'_, '_, '_, '_, KT>,
        add_attribute: &F,
    ) -> Result<(), ExecutionError>
    where
        F: Fn(
            &mut ExecutionContext<'_, '_, '_, '_, KT>,
            Identifier,
            Value,
        ) -> Result<(), ExecutionError>,
    {
        exec.cancellation_flag.check("executing attribute")?;
        let value = self.value.evaluate(exec)?;
        if let Some(shorthand) = exec.shorthands.get(&self.name) {
            shorthand.execute(exec, add_attribute, value)
        } else {
            add_attribute(exec, self.name.clone(), value)
        }
    }
}

impl AttributeShorthand {
    fn execute<F, KT: KindTypes>(
        &self,
        exec: &mut ExecutionContext<'_, '_, '_, '_, KT>,
        add_attribute: &F,
        value: Value,
    ) -> Result<(), ExecutionError>
    where
        F: Fn(
            &mut ExecutionContext<'_, '_, '_, '_, KT>,
            Identifier,
            Value,
        ) -> Result<(), ExecutionError>,
    {
        let mut shorthand_locals = VariableMap::new();
        let mut shorthand_exec = ExecutionContext {
            graph: exec.graph,
            config: exec.config,
            locals: &mut shorthand_locals,
            scoped: exec.scoped,
            current_regex_captures: exec.current_regex_captures,
            function_parameters: exec.function_parameters,
            mat: exec.mat,
            error_context: exec.error_context.clone(),
            inherited_variables: exec.inherited_variables,
            shorthands: exec.shorthands,
            cancellation_flag: exec.cancellation_flag,
        };
        self.variable.add(&mut shorthand_exec, value, false)?;
        for attr in &self.attributes {
            attr.execute(&mut shorthand_exec, add_attribute)?;
        }
        Ok(())
    }
}
