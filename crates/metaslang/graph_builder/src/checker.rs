// -*- coding: utf-8 -*-
// ------------------------------------------------------------------------------------------------
// Copyright © 2022, tree-sitter authors.
// Licensed under either of Apache License, Version 2.0, or MIT license, at your option.
// Please see the LICENSE-APACHE or LICENSE-MIT files in this distribution for license details.
// ------------------------------------------------------------------------------------------------

use std::collections::HashSet;
use std::path::Path;

use metaslang_cst::kinds::KindTypes;
use metaslang_cst::query::CaptureQuantifier::{self, One, OneOrMore, ZeroOrMore, ZeroOrOne};
use metaslang_cst::query::Query;
use thiserror::Error;

use crate::excerpt::Excerpt;
use crate::variables::{MutVariables, VariableError, VariableMap, Variables};
use crate::{ast, Identifier, Location};

#[derive(Debug, Error)]
pub enum CheckError {
    #[error("Cannot hide global variable {0} at {1}")]
    CannotHideGlobalVariable(String, Location),
    #[error("Cannot set global variable {0} at {1}")]
    CannotSetGlobalVariable(String, Location),
    #[error("Duplicate global variable {0} at {1}")]
    DuplicateGlobalVariable(String, Location),
    #[error("Expected list value at {0}")]
    ExpectedListValue(Location),
    #[error("Expected local value at {0}")]
    ExpectedLocalValue(Location),
    #[error("Expected optional value at {0}")]
    ExpectedOptionalValue(Location),
    #[error("Nullable regular expression /{0}/ at {1}")]
    NullableRegex(String, Location),
    #[error("Undefined syntax capture @{0} at {1}")]
    UndefinedSyntaxCapture(String, Location),
    #[error("Undefined variable {0} at {1}")]
    UndefinedVariable(String, Location),
    #[error("Unused capture(s) {0} at {1}. Remove or prefix with _.")]
    UnusedCaptures(String, Location),
    #[error("{0}: {1} at {2}")]
    Variable(VariableError, String, Location),
}

impl CheckError {
    pub fn display_pretty<'a>(
        &'a self,
        path: &'a Path,
        source: &'a str,
    ) -> impl std::fmt::Display + 'a {
        DisplayCheckErrorPretty {
            error: self,
            path,
            source,
        }
    }
}

struct DisplayCheckErrorPretty<'a> {
    error: &'a CheckError,
    path: &'a Path,
    source: &'a str,
}

impl std::fmt::Display for DisplayCheckErrorPretty<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let location = match self.error {
            CheckError::CannotHideGlobalVariable(_, location) => *location,
            CheckError::CannotSetGlobalVariable(_, location) => *location,
            CheckError::DuplicateGlobalVariable(_, location) => *location,
            CheckError::ExpectedListValue(location) => *location,
            CheckError::ExpectedLocalValue(location) => *location,
            CheckError::ExpectedOptionalValue(location) => *location,
            CheckError::NullableRegex(_, location) => *location,
            CheckError::UndefinedSyntaxCapture(_, location) => *location,
            CheckError::UndefinedVariable(_, location) => *location,
            CheckError::UnusedCaptures(_, location) => *location,
            CheckError::Variable(_, _, location) => *location,
        };
        writeln!(f, "{}", self.error)?;
        write!(
            f,
            "{}",
            Excerpt::from_source(
                self.path,
                self.source,
                location.row,
                location.to_column_range(),
                0
            )
        )?;
        Ok(())
    }
}

/// Checker context
struct StanzaCheckContext<'a, KT: KindTypes> {
    stanza_index: usize,
    stanza_query: &'a Query<KT>,
    globals: &'a dyn Variables<VariableResult>,
    locals: &'a mut dyn MutVariables<VariableResult>,
    used_captures: &'a mut HashSet<Identifier>,
}

#[derive(Clone, Debug)]
struct VariableResult {
    is_local: bool,
    quantifier: CaptureQuantifier,
}

//-----------------------------------------------------------------------------
// File

impl<KT: KindTypes> ast::File<KT> {
    pub fn check(&mut self) -> Result<(), CheckError> {
        let mut globals = VariableMap::new();
        for global in &self.globals {
            globals
                .add(
                    global.name.clone(),
                    VariableResult {
                        quantifier: global.quantifier,
                        is_local: true,
                    },
                    false,
                )
                .map_err(|_| {
                    CheckError::DuplicateGlobalVariable(
                        global.name.as_str().to_string(),
                        global.location,
                    )
                })?;
        }
        for (index, stanza) in self.stanzas.iter_mut().enumerate() {
            stanza.check(&globals, index)?;
        }
        Ok(())
    }
}

//-----------------------------------------------------------------------------
// Stanza

impl<KT: KindTypes> ast::Stanza<KT> {
    fn check(
        &mut self,
        globals: &dyn Variables<VariableResult>,
        stanza_index: usize,
    ) -> Result<(), CheckError> {
        let mut locals = VariableMap::new();
        let mut used_captures = HashSet::new();
        let mut ctx = StanzaCheckContext {
            globals,
            stanza_index,
            stanza_query: &self.query,
            locals: &mut locals,
            used_captures: &mut used_captures,
        };
        for statement in &mut self.statements {
            statement.check(&mut ctx)?;
        }

        let all_captures = self
            .query
            .capture_quantifiers()
            .keys()
            .into_iter()
            .map(|cn| Identifier::from(cn.as_str()))
            .collect::<HashSet<_>>();

        let unused_captures = all_captures
            .difference(&ctx.used_captures)
            .filter(|i| !i.starts_with("_"))
            .map(|i| format!("@{}", i))
            .collect::<Vec<_>>();

        if !unused_captures.is_empty() {
            return Err(CheckError::UnusedCaptures(
                unused_captures.join(" "),
                self.range.start,
            ));
        }

        Ok(())
    }
}

//-----------------------------------------------------------------------------
// Statements

impl ast::Statement {
    fn check<KT: KindTypes>(
        &mut self,
        ctx: &mut StanzaCheckContext<'_, KT>,
    ) -> Result<(), CheckError> {
        match self {
            Self::DeclareImmutable(stmt) => stmt.check(ctx),
            Self::DeclareMutable(stmt) => stmt.check(ctx),
            Self::Assign(stmt) => stmt.check(ctx),
            Self::CreateGraphNode(stmt) => stmt.check(ctx),
            Self::AddGraphNodeAttribute(stmt) => stmt.check(ctx),
            Self::CreateEdge(stmt) => stmt.check(ctx),
            Self::AddEdgeAttribute(stmt) => stmt.check(ctx),
            Self::Scan(stmt) => stmt.check(ctx),
            Self::Print(stmt) => stmt.check(ctx),
            Self::If(stmt) => stmt.check(ctx),
            Self::ForIn(stmt) => stmt.check(ctx),
        }
    }
}

impl ast::DeclareImmutable {
    fn check<KT: KindTypes>(
        &mut self,
        ctx: &mut StanzaCheckContext<'_, KT>,
    ) -> Result<(), CheckError> {
        let value = self.value.check(ctx)?;
        self.variable.check_add(ctx, value.into(), false)?;
        Ok(())
    }
}

impl ast::DeclareMutable {
    fn check<KT: KindTypes>(
        &mut self,
        ctx: &mut StanzaCheckContext<'_, KT>,
    ) -> Result<(), CheckError> {
        let value = self.value.check(ctx)?;
        self.variable.check_add(ctx, value.into(), true)?;
        Ok(())
    }
}

impl ast::Assign {
    fn check<KT: KindTypes>(
        &mut self,
        ctx: &mut StanzaCheckContext<'_, KT>,
    ) -> Result<(), CheckError> {
        let value = self.value.check(ctx)?;
        self.variable.check_set(ctx, value.into())?;
        Ok(())
    }
}

impl ast::CreateGraphNode {
    fn check<KT: KindTypes>(
        &mut self,
        ctx: &mut StanzaCheckContext<'_, KT>,
    ) -> Result<(), CheckError> {
        self.node.check_add(
            ctx,
            VariableResult {
                is_local: true,
                quantifier: One,
            },
            false,
        )?;
        Ok(())
    }
}

impl ast::AddGraphNodeAttribute {
    fn check<KT: KindTypes>(
        &mut self,
        ctx: &mut StanzaCheckContext<'_, KT>,
    ) -> Result<(), CheckError> {
        self.node.check(ctx)?;
        for attribute in &mut self.attributes {
            attribute.check(ctx)?;
        }
        Ok(())
    }
}

impl ast::CreateEdge {
    fn check<KT: KindTypes>(
        &mut self,
        ctx: &mut StanzaCheckContext<'_, KT>,
    ) -> Result<(), CheckError> {
        self.source.check(ctx)?;
        self.sink.check(ctx)?;
        Ok(())
    }
}

impl ast::AddEdgeAttribute {
    fn check<KT: KindTypes>(
        &mut self,
        ctx: &mut StanzaCheckContext<'_, KT>,
    ) -> Result<(), CheckError> {
        self.source.check(ctx)?;
        self.sink.check(ctx)?;
        for attribute in &mut self.attributes {
            attribute.check(ctx)?;
        }
        Ok(())
    }
}

impl ast::Scan {
    fn check<KT: KindTypes>(
        &mut self,
        ctx: &mut StanzaCheckContext<'_, KT>,
    ) -> Result<(), CheckError> {
        let value_result = self.value.check(ctx)?;
        if !value_result.is_local {
            return Err(CheckError::ExpectedLocalValue(self.location));
        }

        for arm in &mut self.arms {
            // Be aware that this check is not complete, as it does not rule out
            // all regular expressions that admit empty matches. For example, th
            // regex "\b" matches empty strings within a larger non-empty one.
            // Therefore, there is also a runtime check that checks that a match was
            // non-empty. This is all to prevent non-termination of scan.
            if let Some(_) = arm.regex.captures("") {
                return Err(CheckError::NullableRegex(
                    arm.regex.to_string(),
                    arm.location,
                ));
            }

            let mut arm_locals = VariableMap::nested(ctx.locals);
            let mut arm_ctx = StanzaCheckContext {
                globals: ctx.globals,
                stanza_index: ctx.stanza_index,
                stanza_query: ctx.stanza_query,
                locals: &mut arm_locals,
                used_captures: ctx.used_captures,
            };

            for statement in &mut arm.statements {
                statement.check(&mut arm_ctx)?;
            }
        }
        Ok(())
    }
}

impl ast::Print {
    fn check<KT: KindTypes>(
        &mut self,
        ctx: &mut StanzaCheckContext<'_, KT>,
    ) -> Result<(), CheckError> {
        for value in &mut self.values {
            value.check(ctx)?;
        }
        Ok(())
    }
}

impl ast::If {
    fn check<KT: KindTypes>(
        &mut self,
        ctx: &mut StanzaCheckContext<'_, KT>,
    ) -> Result<(), CheckError> {
        for arm in &mut self.arms {
            for condition in &mut arm.conditions {
                condition.check(ctx)?;
            }

            let mut arm_locals = VariableMap::nested(ctx.locals);
            let mut arm_ctx = StanzaCheckContext {
                globals: ctx.globals,
                stanza_index: ctx.stanza_index,
                stanza_query: ctx.stanza_query,
                locals: &mut arm_locals,
                used_captures: ctx.used_captures,
            };

            for statement in &mut arm.statements {
                statement.check(&mut arm_ctx)?;
            }
        }
        Ok(())
    }
}

impl ast::Condition {
    fn check<KT: KindTypes>(
        &mut self,
        ctx: &mut StanzaCheckContext<'_, KT>,
    ) -> Result<(), CheckError> {
        match self {
            Self::None { value, location } | Self::Some { value, location } => {
                let value_result = value.check(ctx)?;
                if !value_result.is_local {
                    return Err(CheckError::ExpectedLocalValue(*location));
                }
                if value_result.quantifier != ZeroOrOne {
                    return Err(CheckError::ExpectedOptionalValue(*location));
                }
            }
            Self::Bool { value, location } => {
                let value_result = value.check(ctx)?;
                if !value_result.is_local {
                    return Err(CheckError::ExpectedLocalValue(*location));
                }
            }
        }
        Ok(())
    }
}

impl ast::ForIn {
    fn check<KT: KindTypes>(
        &mut self,
        ctx: &mut StanzaCheckContext<'_, KT>,
    ) -> Result<(), CheckError> {
        let value_result = self.value.check(ctx)?;
        if !value_result.is_local {
            return Err(CheckError::ExpectedLocalValue(self.location));
        }
        if value_result.quantifier != ZeroOrMore && value_result.quantifier != OneOrMore {
            return Err(CheckError::ExpectedListValue(self.location));
        }

        let mut loop_locals = VariableMap::nested(ctx.locals);
        let mut loop_ctx = StanzaCheckContext {
            globals: ctx.globals,
            stanza_index: ctx.stanza_index,
            stanza_query: ctx.stanza_query,
            locals: &mut loop_locals,
            used_captures: ctx.used_captures,
        };
        self.variable
            .check_add(&mut loop_ctx, value_result.into(), false)?;

        for statement in &mut self.statements {
            statement.check(&mut loop_ctx)?;
        }

        Ok(())
    }
}

//-----------------------------------------------------------------------------
// Expressions

/// Expression checking result
#[derive(Clone, Debug)]
struct ExpressionResult {
    is_local: bool,
    quantifier: CaptureQuantifier,
}

impl ast::Expression {
    fn check<KT: KindTypes>(
        &mut self,
        ctx: &mut StanzaCheckContext<'_, KT>,
    ) -> Result<ExpressionResult, CheckError> {
        match self {
            Self::FalseLiteral => Ok(ExpressionResult {
                is_local: true,
                quantifier: One,
            }),
            Self::NullLiteral => Ok(ExpressionResult {
                is_local: true,
                quantifier: One,
            }),
            Self::TrueLiteral => Ok(ExpressionResult {
                is_local: true,
                quantifier: One,
            }),
            Self::IntegerConstant(expr) => expr.check(ctx),
            Self::StringConstant(expr) => expr.check(ctx),
            Self::ListLiteral(expr) => expr.check(ctx),
            Self::SetLiteral(expr) => expr.check(ctx),
            Self::ListComprehension(expr) => expr.check(ctx),
            Self::SetComprehension(expr) => expr.check(ctx),
            Self::Capture(expr) => expr.check(ctx),
            Self::Variable(expr) => expr.check_get(ctx),
            Self::Call(expr) => expr.check(ctx),
            Self::RegexCapture(expr) => expr.check(ctx),
        }
    }
}

impl ast::IntegerConstant {
    fn check<KT: KindTypes>(
        &mut self,
        _ctx: &mut StanzaCheckContext<'_, KT>,
    ) -> Result<ExpressionResult, CheckError> {
        Ok(ExpressionResult {
            is_local: true,
            quantifier: One,
        })
    }
}

impl ast::StringConstant {
    fn check<KT: KindTypes>(
        &mut self,
        _ctx: &mut StanzaCheckContext<'_, KT>,
    ) -> Result<ExpressionResult, CheckError> {
        Ok(ExpressionResult {
            is_local: true,
            quantifier: One,
        })
    }
}

impl ast::ListLiteral {
    fn check<KT: KindTypes>(
        &mut self,
        ctx: &mut StanzaCheckContext<'_, KT>,
    ) -> Result<ExpressionResult, CheckError> {
        let mut is_local = true;
        for element in &mut self.elements {
            let element_result = element.check(ctx)?;
            is_local &= element_result.is_local;
        }
        Ok(ExpressionResult {
            is_local,
            quantifier: ZeroOrMore,
        })
    }
}

impl ast::SetLiteral {
    fn check<KT: KindTypes>(
        &mut self,
        ctx: &mut StanzaCheckContext<'_, KT>,
    ) -> Result<ExpressionResult, CheckError> {
        let mut is_local = true;
        for element in &mut self.elements {
            let element_result = element.check(ctx)?;
            is_local &= element_result.is_local;
        }
        Ok(ExpressionResult {
            is_local,
            quantifier: ZeroOrMore,
        })
    }
}

impl ast::ListComprehension {
    fn check<KT: KindTypes>(
        &mut self,
        ctx: &mut StanzaCheckContext<'_, KT>,
    ) -> Result<ExpressionResult, CheckError> {
        let value_result = self.value.check(ctx)?;
        if !value_result.is_local {
            return Err(CheckError::ExpectedLocalValue(self.location));
        }
        if value_result.quantifier != ZeroOrMore && value_result.quantifier != OneOrMore {
            return Err(CheckError::ExpectedListValue(self.location));
        }

        let mut loop_locals = VariableMap::nested(ctx.locals);
        let mut loop_ctx = StanzaCheckContext {
            globals: ctx.globals,
            stanza_index: ctx.stanza_index,
            stanza_query: ctx.stanza_query,
            locals: &mut loop_locals,
            used_captures: ctx.used_captures,
        };
        self.variable
            .check_add(&mut loop_ctx, value_result.into(), false)?;

        let element_result = self.element.check(&mut loop_ctx)?;

        Ok(ExpressionResult {
            is_local: element_result.is_local,
            quantifier: ZeroOrMore,
        })
    }
}

impl ast::SetComprehension {
    fn check<KT: KindTypes>(
        &mut self,
        ctx: &mut StanzaCheckContext<'_, KT>,
    ) -> Result<ExpressionResult, CheckError> {
        let value_result = self.value.check(ctx)?;
        if !value_result.is_local {
            return Err(CheckError::ExpectedLocalValue(self.location));
        }
        if value_result.quantifier != ZeroOrMore && value_result.quantifier != OneOrMore {
            return Err(CheckError::ExpectedListValue(self.location));
        }

        let mut loop_locals = VariableMap::nested(ctx.locals);
        let mut loop_ctx = StanzaCheckContext {
            globals: ctx.globals,
            stanza_index: ctx.stanza_index,
            stanza_query: ctx.stanza_query,
            locals: &mut loop_locals,
            used_captures: ctx.used_captures,
        };
        self.variable
            .check_add(&mut loop_ctx, value_result.into(), false)?;

        let element_result = self.element.check(&mut loop_ctx)?;

        Ok(ExpressionResult {
            is_local: element_result.is_local,
            quantifier: ZeroOrMore,
        })
    }
}

impl ast::Capture {
    fn check<KT: KindTypes>(
        &mut self,
        ctx: &mut StanzaCheckContext<'_, KT>,
    ) -> Result<ExpressionResult, CheckError> {
        let name = self.name.to_string();
        if let Some(quantifier) = ctx.stanza_query.capture_quantifiers().get(&name) {
            self.quantifier = *quantifier;
            ctx.used_captures.insert(self.name.clone());
            Ok(ExpressionResult {
                is_local: true,
                quantifier: self.quantifier,
            })
        } else {
            Err(CheckError::UndefinedSyntaxCapture(
                name.clone(),
                self.location,
            ))
        }
    }
}

impl ast::Call {
    fn check<KT: KindTypes>(
        &mut self,
        ctx: &mut StanzaCheckContext<'_, KT>,
    ) -> Result<ExpressionResult, CheckError> {
        let mut is_local = true;
        for parameter in &mut self.parameters {
            let parameter_result = parameter.check(ctx)?;
            is_local &= parameter_result.is_local;
        }
        Ok(ExpressionResult {
            is_local,
            quantifier: One, // FIXME we don't really know
        })
    }
}

impl ast::RegexCapture {
    fn check<KT: KindTypes>(
        &mut self,
        _ctx: &mut StanzaCheckContext<'_, KT>,
    ) -> Result<ExpressionResult, CheckError> {
        Ok(ExpressionResult {
            is_local: true,
            quantifier: One,
        })
    }
}

//-----------------------------------------------------------------------------
// Variables

impl ast::Variable {
    fn check_add<KT: KindTypes>(
        &mut self,
        ctx: &mut StanzaCheckContext<'_, KT>,
        value: VariableResult,
        mutable: bool,
    ) -> Result<(), CheckError> {
        match self {
            Self::Unscoped(v) => v.check_add(ctx, value, mutable),
            Self::Scoped(v) => v.check_add(ctx, value, mutable),
        }
    }

    fn check_set<KT: KindTypes>(
        &mut self,
        ctx: &mut StanzaCheckContext<'_, KT>,
        value: VariableResult,
    ) -> Result<(), CheckError> {
        match self {
            Self::Unscoped(v) => v.check_set(ctx, value),
            Self::Scoped(v) => v.check_set(ctx, value),
        }
    }

    fn check_get<KT: KindTypes>(
        &mut self,
        ctx: &mut StanzaCheckContext<'_, KT>,
    ) -> Result<ExpressionResult, CheckError> {
        match self {
            Self::Unscoped(v) => v.check_get(ctx),
            Self::Scoped(v) => v.check_get(ctx),
        }
    }
}

impl ast::UnscopedVariable {
    fn check_add<KT: KindTypes>(
        &mut self,
        ctx: &mut StanzaCheckContext<'_, KT>,
        value: VariableResult,
        mutable: bool,
    ) -> Result<(), CheckError> {
        if ctx.globals.get(&self.name).is_some() {
            return Err(CheckError::CannotHideGlobalVariable(
                self.name.as_str().to_string(),
                self.location,
            ));
        }
        let mut value = value;
        // Mutable variables are not considered local, because a non-local
        // assignment in a loop could invalidate an earlier local assignment.
        // Since we process all statement in order, we don't have info on later
        // assignments, and can assume non-local to be sound.
        if mutable {
            value.is_local = false;
        }
        ctx.locals
            .add(self.name.clone(), value, mutable)
            .map_err(|e| CheckError::Variable(e, format!("{}", self.name), self.location))?;
        Ok(())
    }

    fn check_set<KT: KindTypes>(
        &mut self,
        ctx: &mut StanzaCheckContext<'_, KT>,
        value: VariableResult,
    ) -> Result<(), CheckError> {
        if ctx.globals.get(&self.name).is_some() {
            return Err(CheckError::CannotSetGlobalVariable(
                self.name.as_str().to_string(),
                self.location,
            ));
        }
        let mut value = value;
        // Mutable variables are not considered local, because a non-local
        // assignment in a loop could invalidate an earlier local assignment.
        // Since we process all statement in order, we don't have info on later
        // assignments, and can assume non-local to be sound.
        value.is_local = false;
        ctx.locals
            .set(self.name.clone(), value)
            .map_err(|e| CheckError::Variable(e, format!("{}", self.name), self.location))?;
        Ok(())
    }

    fn check_get<KT: KindTypes>(
        &mut self,
        ctx: &mut StanzaCheckContext<'_, KT>,
    ) -> Result<ExpressionResult, CheckError> {
        if let Some(result) = ctx.globals.get(&self.name) {
            Some(result)
        } else {
            ctx.locals.get(&self.name)
        }
        .map(|value| value.into())
        .ok_or_else(|| CheckError::UndefinedVariable(self.name.as_str().to_string(), self.location))
    }
}

impl ast::ScopedVariable {
    fn check_add<KT: KindTypes>(
        &mut self,
        ctx: &mut StanzaCheckContext<'_, KT>,
        _value: VariableResult,
        _mutable: bool,
    ) -> Result<(), CheckError> {
        self.scope.check(ctx)?;
        Ok(())
    }

    fn check_set<KT: KindTypes>(
        &mut self,
        ctx: &mut StanzaCheckContext<'_, KT>,
        _value: VariableResult,
    ) -> Result<(), CheckError> {
        self.scope.check(ctx)?;
        Ok(())
    }

    fn check_get<KT: KindTypes>(
        &mut self,
        ctx: &mut StanzaCheckContext<'_, KT>,
    ) -> Result<ExpressionResult, CheckError> {
        self.scope.check(ctx)?;
        Ok(ExpressionResult {
            is_local: false,
            quantifier: One, // FIXME we don't really know
        })
    }
}

//-----------------------------------------------------------------------------
// Attributes

impl ast::Attribute {
    fn check<KT: KindTypes>(
        &mut self,
        ctx: &mut StanzaCheckContext<'_, KT>,
    ) -> Result<(), CheckError> {
        self.value.check(ctx)?;
        Ok(())
    }
}

//-----------------------------------------------------------------------------
// Result Conversions

impl Into<ExpressionResult> for &VariableResult {
    fn into(self) -> ExpressionResult {
        ExpressionResult {
            is_local: self.is_local,
            quantifier: self.quantifier,
        }
    }
}

impl Into<VariableResult> for ExpressionResult {
    fn into(self) -> VariableResult {
        VariableResult {
            is_local: self.is_local,
            quantifier: self.quantifier,
        }
    }
}
