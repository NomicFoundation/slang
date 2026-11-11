use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::diagnostics::kinds::DiagnosticKind;
use slang_solidity_v2_common::diagnostics::kinds::type_system::{
    InvalidArgumentType, UnknownNamedArgument, WrongArgumentCount,
};
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;
use slang_solidity_v2_ir::ir::{NodeIdentity, TextRange};

use super::Pass;
use crate::binder::{Binder, ParameterDefinition, Scope, ScopeId};
use crate::context::FileNodeMapper;
use crate::passes::common::node_location;
use crate::types::{FunctionType, Type, TypeId, TypeRegistry};

/// The ordered parameters that the arguments of a call are checked against,
/// with the name and type of each one where known.
pub(super) trait ParameterList {
    fn count(&self) -> usize;

    /// The name of the parameter at `index`, if it has one.
    fn name(&self, index: usize) -> Option<&str>;

    /// The type of the parameter at `index`, if it resolved.
    fn type_id(&self, index: usize) -> Option<TypeId>;

    /// The position of the parameter named `name`, if any.
    fn position_of(&self, name: &str) -> Option<usize> {
        (0..self.count()).find(|&index| self.name(index) == Some(name))
    }
}

/// The parameters of a declaration, eg. a function.
impl ParameterList for [ParameterDefinition] {
    fn count(&self) -> usize {
        self.len()
    }

    fn name(&self, index: usize) -> Option<&str> {
        self[index].name.as_deref()
    }

    fn type_id(&self, index: usize) -> Option<TypeId> {
        self[index].type_id
    }
}

/// Parameters known only by their types, eg. those of a function value.
impl ParameterList for [TypeId] {
    fn count(&self) -> usize {
        self.len()
    }

    fn name(&self, _index: usize) -> Option<&str> {
        None
    }

    fn type_id(&self, index: usize) -> Option<TypeId> {
        Some(self[index])
    }
}

impl Pass<'_> {
    /// Checks the positional `arguments` of a call to the function type
    /// `function_type_id` against its parameters. `argument_types` is `None`
    /// when an argument has no type, which is already reported.
    pub(super) fn check_positional_arguments_of_function_call(
        &mut self,
        node: &ir::FunctionCallExpression,
        function_type_id: TypeId,
        arguments: &[ir::Expression],
        argument_types: Option<&[TypeId]>,
    ) {
        let (binder, mut checker) = self.arguments_checker();
        let Type::Function(function_type) = checker.types.get_type_by_id(function_type_id) else {
            unreachable!("arguments are checked against a function type");
        };
        match declared_parameters(binder, function_type.definition_id) {
            Some(parameters) => checker.check_positional(
                node,
                unbound(function_type, parameters),
                crosses_external_boundary(function_type),
                arguments,
                argument_types,
            ),
            // A function value has only the types of its parameters.
            None => checker.check_positional(
                node,
                unbound(function_type, &function_type.parameter_types),
                false,
                arguments,
                argument_types,
            ),
        }
    }

    /// Checks the named `arguments` of a call to the function type
    /// `function_type_id` against its parameters. `argument_types` is `None`
    /// when an argument has no type, which is already reported.
    pub(super) fn check_named_arguments_of_function_call(
        &mut self,
        node: &ir::FunctionCallExpression,
        function_type_id: TypeId,
        arguments: &[ir::NamedArgument],
        argument_types: Option<&[(&str, TypeId)]>,
    ) {
        let (binder, mut checker) = self.arguments_checker();
        let Type::Function(function_type) = checker.types.get_type_by_id(function_type_id) else {
            unreachable!("arguments are checked against a function type");
        };
        match declared_parameters(binder, function_type.definition_id) {
            Some(parameters) => checker.check_named(
                node,
                unbound(function_type, parameters),
                crosses_external_boundary(function_type),
                arguments,
                argument_types,
            ),
            // A function value has no parameter names to match.
            None => checker.check_named(
                node,
                unbound(function_type, &function_type.parameter_types),
                false,
                arguments,
                argument_types,
            ),
        }
    }

    /// Splits the pass into the binder, which parameters are read from, and a
    /// checker that reports on the arguments.
    fn arguments_checker(&mut self) -> (&Binder, ArgumentsChecker<'_>) {
        let checker = ArgumentsChecker {
            types: self.types,
            file_node_mapper: self.file_node_mapper,
            diagnostics: self.diagnostics,
        };
        (self.binder, checker)
    }
}

/// Checks the arguments of a call against a [`ParameterList`]. It borrows only
/// what the checks need from the pass, so the parameters can be borrowed from
/// the binder or the type registry alongside it.
struct ArgumentsChecker<'a> {
    types: &'a TypeRegistry,
    file_node_mapper: &'a FileNodeMapper,
    diagnostics: &'a mut DiagnosticCollection,
}

impl ArgumentsChecker<'_> {
    /// Checks positional `arguments` against `parameters`: their count, and
    /// that each argument converts to the type of the parameter in its
    /// position. When an argument has no type, only the count is checked.
    fn check_positional(
        &mut self,
        node: &ir::FunctionCallExpression,
        parameters: &(impl ParameterList + ?Sized),
        external_call: bool,
        arguments: &[ir::Expression],
        argument_types: Option<&[TypeId]>,
    ) {
        // 1. There is one argument per parameter.
        if arguments.len() != parameters.count() {
            let kind = WrongArgumentCount {
                given: arguments.len(),
                expected: parameters.count(),
            };
            self.report(node, kind);
            return;
        }

        // 2. Each argument converts to the type of the parameter in its
        // position.
        let Some(argument_types) = argument_types else {
            return;
        };
        for (index, argument) in arguments.iter().enumerate() {
            if !self.converts(
                argument_types[index],
                parameters.type_id(index),
                external_call,
            ) {
                self.report(argument, InvalidArgumentType);
            }
        }
    }

    /// Checks named `arguments` against `parameters`: their count, that each
    /// argument names a parameter, and that each argument converts to the type
    /// of the parameter it names. When an argument has no type, the types are
    /// not checked.
    fn check_named(
        &mut self,
        node: &ir::FunctionCallExpression,
        parameters: &(impl ParameterList + ?Sized),
        external_call: bool,
        arguments: &[ir::NamedArgument],
        argument_types: Option<&[(&str, TypeId)]>,
    ) {
        // 1. There is one argument per parameter.
        if arguments.len() != parameters.count() {
            let kind = WrongArgumentCount {
                given: arguments.len(),
                expected: parameters.count(),
            };
            self.report(node, kind);
            return;
        }

        // 2. Each argument names a parameter. Matching types to the wrong
        // parameters would only cascade, so the types are not checked otherwise.
        let mut parameter_indices = Vec::with_capacity(arguments.len());
        let mut all_named = true;
        for argument in arguments {
            let name = argument.name.unparse();
            if let Some(parameter_index) = parameters.position_of(name) {
                parameter_indices.push(parameter_index);
            } else {
                let kind = UnknownNamedArgument {
                    name: name.to_owned(),
                };
                self.report(&argument.name, kind);
                all_named = false;
            }
        }
        if !all_named {
            return;
        }

        // 3. Each argument converts to the type of the parameter it names.
        let Some(argument_types) = argument_types else {
            return;
        };
        for (index, argument) in arguments.iter().enumerate() {
            let parameter_type = parameters.type_id(parameter_indices[index]);
            if !self.converts(argument_types[index].1, parameter_type, external_call) {
                self.report(&argument.value, InvalidArgumentType);
            }
        }
    }

    /// Whether `argument_type` converts to `parameter_type`. A parameter
    /// without a type is already reported, so any argument converts to it.
    fn converts(
        &self,
        argument_type: TypeId,
        parameter_type: Option<TypeId>,
        external_call: bool,
    ) -> bool {
        parameter_type.is_none_or(|parameter_type| {
            parameter_type_matches_argument_type(
                self.types,
                parameter_type,
                argument_type,
                external_call,
            )
        })
    }

    fn report(&mut self, node: &(impl NodeIdentity + TextRange), kind: impl Into<DiagnosticKind>) {
        let (file_id, range) = node_location(node, self.file_node_mapper);
        self.diagnostics.push(file_id, range, kind);
    }
}

/// Whether `argument_type` implicitly converts to `parameter_type`. An
/// `external_call` crosses an external boundary, which relaxes the data
/// location rules, eg. a calldata parameter accepts a memory argument.
pub(super) fn parameter_type_matches_argument_type(
    types: &TypeRegistry,
    parameter_type: TypeId,
    argument_type: TypeId,
    external_call: bool,
) -> bool {
    types.implicitly_convertible_to_for_call(argument_type, parameter_type, external_call)
}

/// Whether a call to `function_type` passes its declared parameters across an
/// external boundary: an external or a delegate call.
fn crosses_external_boundary(function_type: &FunctionType) -> bool {
    function_type.call_mode.is_external() || function_type.call_mode.is_delegate()
}

/// The parameters declared by `definition_id`, if it declares any, eg. a
/// function or a getter.
// TODO: replace this and `parameters_of_scope` with
// `Binder::get_parameters_for_definition` once #2370 is merged.
pub(super) fn declared_parameters(
    binder: &Binder,
    definition_id: Option<NodeId>,
) -> Option<&[ParameterDefinition]> {
    let parameters_scope_id = binder.get_parameters_scope_for_definition(definition_id?)?;
    Some(parameters_of_scope(binder, parameters_scope_id))
}

fn parameters_of_scope(binder: &Binder, parameters_scope_id: ScopeId) -> &[ParameterDefinition] {
    let Scope::Parameters(parameters_scope) = binder.get_scope_by_id(parameters_scope_id) else {
        unreachable!("incorrect scope kind, expected parameters");
    };
    &parameters_scope.parameters
}

/// The part of `parameters` (of `function_type`) that a call passes: a bound
/// function already has its first argument, which cannot be named either.
fn unbound<'a, T>(function_type: &FunctionType, parameters: &'a [T]) -> &'a [T] {
    let skip = usize::from(function_type.call_mode.is_bound());
    parameters.get(skip..).unwrap_or_default()
}
