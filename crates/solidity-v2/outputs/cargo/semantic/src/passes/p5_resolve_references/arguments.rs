use std::sync::Arc;

use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::diagnostics::kinds::DiagnosticKind;
use slang_solidity_v2_common::diagnostics::kinds::resolution::NoMatchingCallableDeclaration;
use slang_solidity_v2_common::diagnostics::kinds::semantic::BaseConstructorCallWithoutParentheses;
use slang_solidity_v2_common::diagnostics::kinds::type_system::{
    EncodedArgumentsNotInlineTuple, InvalidArgumentType, InvalidDecodedTypes,
    InvalidEncodedFunction, NamedArgumentsToBuiltInFunction, StructWithMappingConstructed,
    TooFewArguments, UnknownNamedArgument, WrongArgumentCount,
};
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_ir::ir;
use slang_solidity_v2_ir::ir::{NodeIdentity, TextRange};

use super::Pass;
use crate::binder::{Binder, Definition, ParameterDefinition, Resolution, Scope};
use crate::built_ins::{BuiltInOverloadMatch, InternalBuiltIn};
use crate::context::FileNodeMapper;
use crate::passes::common::node_location;
use crate::types::{
    CallMode, ContractType, DataLocation, FunctionType, TupleType, Type, TypeId, TypeRegistry,
    UserMetaType,
};

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

/// The parameters of a declaration, eg. a function, a constructor, an event
/// or an error.
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

/// The members of a struct, as the parameters of its construction. Each one has
/// its type in memory, where the struct is constructed.
struct StructMembers {
    members: ir::StructMembers,
    type_ids: Vec<Option<TypeId>>,
}

impl ParameterList for StructMembers {
    fn count(&self) -> usize {
        self.members.len()
    }

    fn name(&self, index: usize) -> Option<&str> {
        Some(self.members[index].name.unparse())
    }

    fn type_id(&self, index: usize) -> Option<TypeId> {
        self.type_ids[index]
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
                without_bound_receiver(parameters, function_type.call_mode),
                function_type.call_mode.crosses_external_boundary(),
                arguments,
                argument_types,
            ),
            // A function value or a getter has the types at the call itself.
            // TODO: review whether an external function value should relax
            // the data location rules like an external call to a declared
            // function does, eg. accept a memory argument for a calldata one.
            // `external_call` is false for now for compatibility with solc.
            None => checker.check_positional(
                node,
                without_bound_receiver(&function_type.parameter_types, function_type.call_mode),
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
                without_bound_receiver(parameters, function_type.call_mode),
                function_type.call_mode.crosses_external_boundary(),
                arguments,
                argument_types,
            ),
            // A function value or a getter has no parameter names to match.
            // TODO: review whether an external function value should relax
            // the data location rules like an external call to a declared
            // function does, eg. accept a memory argument for a calldata one.
            // `external_call` is false for now for compatibility with solc.
            None => checker.check_named(
                node,
                without_bound_receiver(&function_type.parameter_types, function_type.call_mode),
                false,
                arguments,
                argument_types,
            ),
        }
    }

    /// Checks the positional `arguments` of creating a value of
    /// `created_type_id` with `new` against the parameters of the creation.
    pub(super) fn check_positional_arguments_of_creation(
        &mut self,
        node: &ir::FunctionCallExpression,
        created_type_id: TypeId,
        arguments: &[ir::Expression],
        argument_types: Option<&[TypeId]>,
    ) {
        let (binder, mut checker) = self.arguments_checker();
        match creation_parameters(binder, checker.types, created_type_id) {
            Some(CreationParameters::Constructor(parameters)) => {
                checker.check_positional(node, parameters, false, arguments, argument_types);
            }
            Some(CreationParameters::Length(length)) => {
                checker.check_positional(node, &[length][..], false, arguments, argument_types);
            }
            None => {}
        }
    }

    /// Checks the named `arguments` of creating a value of `created_type_id`
    /// with `new` against the parameters of the creation.
    pub(super) fn check_named_arguments_of_creation(
        &mut self,
        node: &ir::FunctionCallExpression,
        created_type_id: TypeId,
        arguments: &[ir::NamedArgument],
        argument_types: Option<&[(&str, TypeId)]>,
    ) {
        let (binder, mut checker) = self.arguments_checker();
        match creation_parameters(binder, checker.types, created_type_id) {
            Some(CreationParameters::Constructor(parameters)) => {
                checker.check_named(node, parameters, false, arguments, argument_types);
            }
            Some(CreationParameters::Length(length)) => {
                checker.check_named(node, &[length][..], false, arguments, argument_types);
            }
            None => {}
        }
    }

    /// Checks the positional `arguments` of a use of the event or error
    /// `definition_id`, eg. an `emit` or `revert` statement, or an error
    /// construction, against its parameters. A wrong argument count is
    /// reported at `node`.
    pub(super) fn check_positional_arguments_of_event_or_error(
        &mut self,
        node: &(impl NodeIdentity + TextRange),
        definition_id: NodeId,
        arguments: &[ir::Expression],
        argument_types: Option<&[TypeId]>,
    ) {
        let (binder, mut checker) = self.arguments_checker();
        if let Some(parameters) = declared_parameters(binder, Some(definition_id)) {
            checker.check_positional(node, parameters, false, arguments, argument_types);
        }
    }

    /// Checks the named `arguments` of a use of the event or error
    /// `definition_id`, eg. an `emit` or `revert` statement, or an error
    /// construction, against its parameters. A wrong argument count is
    /// reported at `node`.
    pub(super) fn check_named_arguments_of_event_or_error(
        &mut self,
        node: &(impl NodeIdentity + TextRange),
        definition_id: NodeId,
        arguments: &[ir::NamedArgument],
        argument_types: Option<&[(&str, TypeId)]>,
    ) {
        let (binder, mut checker) = self.arguments_checker();
        if let Some(parameters) = declared_parameters(binder, Some(definition_id)) {
            checker.check_named(node, parameters, false, arguments, argument_types);
        }
    }

    /// Checks the positional `arguments` of constructing the struct
    /// `definition_id`, eg. `S(1, 2)`, against its members.
    pub(super) fn check_positional_arguments_of_struct_construction(
        &mut self,
        node: &ir::FunctionCallExpression,
        definition_id: NodeId,
        arguments: &[ir::Expression],
        argument_types: Option<&[TypeId]>,
    ) {
        let Some(members) = self.constructed_struct_members(node, definition_id) else {
            return;
        };
        let (_, mut checker) = self.arguments_checker();
        checker.check_positional(node, &members, false, arguments, argument_types);
    }

    /// Checks the named `arguments` of constructing the struct
    /// `definition_id`, eg. `S({a: 1, b: 2})`, against its members.
    pub(super) fn check_named_arguments_of_struct_construction(
        &mut self,
        node: &ir::FunctionCallExpression,
        definition_id: NodeId,
        arguments: &[ir::NamedArgument],
        argument_types: Option<&[(&str, TypeId)]>,
    ) {
        let Some(members) = self.constructed_struct_members(node, definition_id) else {
            return;
        };
        let (_, mut checker) = self.arguments_checker();
        checker.check_named(node, &members, false, arguments, argument_types);
    }

    /// Checks the positional `arguments` of a call to `built_in` against its
    /// parameters, if it has a fixed signature or leading parameters, or
    /// against its overloads.
    pub(super) fn check_positional_arguments_of_built_in_call(
        &mut self,
        node: &ir::FunctionCallExpression,
        built_in: &InternalBuiltIn,
        arguments: &[ir::Expression],
        argument_types: Option<&[TypeId]>,
    ) {
        match built_in {
            InternalBuiltIn::AbiDecode => {
                self.check_arguments_of_abi_decode(node, arguments, argument_types);
                return;
            }
            InternalBuiltIn::AbiEncodeCall => {
                self.check_arguments_of_abi_encode_call(node, arguments, argument_types);
                return;
            }
            _ => {}
        }
        if let Some(parameter_types) = self.built_ins_resolver().parameter_types_of(built_in) {
            let (_, mut checker) = self.arguments_checker();
            checker.check_positional(node, &parameter_types[..], false, arguments, argument_types);
            return;
        }
        if let Some(parameter_types) = self
            .built_ins_resolver()
            .leading_parameter_types_of(built_in)
        {
            let (_, mut checker) = self.arguments_checker();
            checker.check_leading_positional(node, &parameter_types[..], arguments, argument_types);
            return;
        }
        // An argument without a type, which is already reported, cannot match
        // an overload.
        let Some(argument_types) = argument_types else {
            return;
        };
        let overload_match = self
            .built_ins_resolver()
            .match_overload(built_in, argument_types);
        if overload_match == BuiltInOverloadMatch::Rejected {
            self.push_diagnostic(node, NoMatchingCallableDeclaration);
        }
    }

    /// Checks the `arguments` of `abi.decode(data, (T1, T2, ...))`: there are
    /// two, the first is the bytes to decode, and the second is a
    /// parenthesized type name or tuple of type names to decode them to.
    fn check_arguments_of_abi_decode(
        &mut self,
        node: &ir::FunctionCallExpression,
        arguments: &[ir::Expression],
        argument_types: Option<&[TypeId]>,
    ) {
        let [data, types] = arguments else {
            let (_, mut checker) = self.arguments_checker();
            let kind = WrongArgumentCount {
                given: arguments.len(),
                expected: 2,
            };
            checker.report(node, kind);
            return;
        };
        let Some([data_type, types_type]) = argument_types else {
            return;
        };
        let decodes = matches!(types, ir::Expression::TupleExpression(_))
            && self
                .built_ins_resolver()
                .decoded_type_of(*types_type)
                .is_some();
        let (_, mut checker) = self.arguments_checker();
        if !checker.parameter_accepts_argument(
            Some(checker.types.bytes_memory()),
            *data_type,
            false,
        ) {
            checker.report(data, InvalidArgumentType);
        }
        if !decodes {
            checker.report(types, InvalidDecodedTypes);
        }
    }

    /// Checks the `arguments` of `abi.encodeCall(f, (a, b, ...))`: there are
    /// two, the first is an external function, and the second is the arguments
    /// to encode a call to it with, either an inline tuple or a single value.
    fn check_arguments_of_abi_encode_call(
        &mut self,
        node: &ir::FunctionCallExpression,
        arguments: &[ir::Expression],
        argument_types: Option<&[TypeId]>,
    ) {
        let (binder, mut checker) = self.arguments_checker();
        let [function, encoded] = arguments else {
            let kind = WrongArgumentCount {
                given: arguments.len(),
                expected: 2,
            };
            checker.report(node, kind);
            return;
        };
        let Some([function_type, encoded_type]) = argument_types else {
            return;
        };
        let Some(parameter_types) =
            encoded_function_parameter_types(binder, checker.types, *function_type)
        else {
            checker.report(function, InvalidEncodedFunction);
            return;
        };

        // The components of an inline tuple are the arguments, except that a
        // single one types as itself. Any other expression is a single
        // argument, which cannot be a tuple.
        let encoded_types = checker.types.get_type_by_id(*encoded_type);
        let (components, component_types) = match encoded {
            ir::Expression::TupleExpression(tuple) => {
                // An omitted component is already reported.
                let Some(components) = tuple
                    .items
                    .iter()
                    .map(|item| item.expression.clone())
                    .collect::<Option<Vec<_>>>()
                else {
                    return;
                };
                let component_types = match encoded_types {
                    Type::Tuple(TupleType { types }) if components.len() != 1 => types.clone(),
                    _ => vec![*encoded_type],
                };
                (components, component_types)
            }
            _ if matches!(encoded_types, Type::Tuple(_)) => {
                checker.report(encoded, EncodedArgumentsNotInlineTuple);
                return;
            }
            _ => (vec![encoded.clone()], vec![*encoded_type]),
        };
        checker.check_positional(
            encoded,
            parameter_types,
            true,
            &components,
            Some(&component_types),
        );
    }

    /// Checks the named `arguments` of a call to `built_in`. The parameters of
    /// a built-in have no names, so only an empty list is accepted.
    pub(super) fn check_named_arguments_of_built_in_call(
        &mut self,
        node: &ir::FunctionCallExpression,
        built_in: &InternalBuiltIn,
        arguments: &[ir::NamedArgument],
    ) {
        // Without any arguments, the call is the same as a positional one.
        if arguments.is_empty() {
            self.check_positional_arguments_of_built_in_call(node, built_in, &[], Some(&[]));
        } else {
            self.push_diagnostic(node, NamedArgumentsToBuiltInFunction);
        }
    }

    /// Checks the `arguments` of the modifier invocation `node` against the
    /// parameters of the modifier, or on a constructor, of the base
    /// constructor it names. A base constructor call without parentheses is
    /// reported instead.
    pub(super) fn check_arguments_of_modifier_invocation(
        &mut self,
        node: &ir::ModifierInvocation,
        arguments: &[ir::Expression],
        argument_types: Option<&[TypeId]>,
    ) {
        let identifier = node.name.last().expect("an identifier path is not empty");
        let Some(Resolution::Definition(definition_id)) =
            self.followed_resolution_of_reference(identifier.id())
        else {
            return;
        };
        let (binder, mut checker) = self.arguments_checker();
        if let Some(Definition::Modifier(_)) = binder.find_definition_by_id(definition_id) {
            let Some(parameters) = declared_parameters(binder, Some(definition_id)) else {
                unreachable!("a modifier declares its parameters");
            };
            checker.check_positional(node, parameters, false, arguments, argument_types);
        } else if let Some(parameters) = base_constructor_parameters(binder, definition_id) {
            // Even without arguments, a base constructor call needs
            // parentheses. The missing arguments are not checked.
            if node.arguments.is_none() {
                checker.report(node, BaseConstructorCallWithoutParentheses);
            } else {
                checker.check_positional(node, parameters, false, arguments, argument_types);
            }
        }
    }

    /// Checks the `arguments` of the base `node` in an inheritance list, eg.
    /// `is B(1)`, against the parameters of its constructor.
    pub(super) fn check_arguments_of_inheritance_type(
        &mut self,
        node: &ir::InheritanceType,
        arguments: &[ir::Expression],
        argument_types: Option<&[TypeId]>,
    ) {
        let identifier = node
            .type_name
            .last()
            .expect("an identifier path is not empty");
        let Some(Resolution::Definition(definition_id)) =
            self.followed_resolution_of_reference(identifier.id())
        else {
            return;
        };
        let (binder, mut checker) = self.arguments_checker();
        if let Some(parameters) = base_constructor_parameters(binder, definition_id) {
            checker.check_positional(node, parameters, false, arguments, argument_types);
        }
    }

    /// The members of the struct `definition_id`, with their types in memory,
    /// where the struct is constructed. Returns `None`, reported at `node`,
    /// when the struct cannot be constructed at all.
    fn constructed_struct_members(
        &mut self,
        node: &ir::FunctionCallExpression,
        definition_id: NodeId,
    ) -> Option<StructMembers> {
        let Some(Definition::Struct(struct_definition)) =
            self.binder.find_definition_by_id(definition_id)
        else {
            unreachable!("a struct construction names a struct");
        };
        // A mapping has no memory form, so neither has the struct, and its
        // arguments are not checked.
        if struct_definition.contains_mapping {
            self.push_diagnostic(node, StructWithMappingConstructed);
            return None;
        }
        let members = Arc::clone(&struct_definition.ir_node.members);
        let type_ids = members
            .iter()
            .map(|member| {
                let type_id = self.binder.node_typing(member.id()).as_type_id()?;
                let type_ = self.types.get_type_by_id(type_id);
                if !type_.is_inherited_location() {
                    return Some(type_id);
                }
                let type_ = type_.clone();
                Some(
                    self.types
                        .register_type_with_data_location(type_, DataLocation::Memory),
                )
            })
            .collect();
        Some(StructMembers { members, type_ids })
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
        node: &(impl NodeIdentity + TextRange),
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
            if !self.parameter_accepts_argument(
                parameters.type_id(index),
                argument_types[index],
                external_call,
            ) {
                self.report(argument, InvalidArgumentType);
            }
        }
    }

    /// Checks the leading positional `arguments` against `parameters`, for a
    /// callee that takes any number of arguments after them: their count, and
    /// that each converts to the type of the parameter in its position.
    fn check_leading_positional(
        &mut self,
        node: &(impl NodeIdentity + TextRange),
        parameters: &(impl ParameterList + ?Sized),
        arguments: &[ir::Expression],
        argument_types: Option<&[TypeId]>,
    ) {
        // 1. There is at least one argument per parameter.
        if arguments.len() < parameters.count() {
            let kind = TooFewArguments {
                given: arguments.len(),
                minimum: parameters.count(),
            };
            self.report(node, kind);
            return;
        }

        // 2. Each leading argument converts to the type of the parameter in
        // its position.
        let Some(argument_types) = argument_types else {
            return;
        };
        for (index, argument) in arguments.iter().take(parameters.count()).enumerate() {
            if !self.parameter_accepts_argument(
                parameters.type_id(index),
                argument_types[index],
                false,
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
        node: &(impl NodeIdentity + TextRange),
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
            if !self.parameter_accepts_argument(
                parameter_type,
                argument_types[index].1,
                external_call,
            ) {
                self.report(&argument.value, InvalidArgumentType);
            }
        }
    }

    /// Whether a parameter of `parameter_type` accepts an argument of
    /// `argument_type`. A parameter without a type is already reported, so it
    /// accepts any argument.
    fn parameter_accepts_argument(
        &self,
        parameter_type: Option<TypeId>,
        argument_type: TypeId,
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

/// The parameter types of the function `abi.encodeCall` encodes a call to,
/// given the type of its first argument: an external function value, or from
/// 0.8.12, an externally visible function named through a contract or
/// interface, eg. `I.f`. Returns `None` for anything else.
fn encoded_function_parameter_types<'a>(
    binder: &Binder,
    types: &'a TypeRegistry,
    type_id: TypeId,
) -> Option<&'a [TypeId]> {
    let type_id = match types.get_type_by_id(type_id) {
        Type::UserMetaType(UserMetaType { definition_id })
            if types.language_version() >= LanguageVersion::V0_8_12 =>
        {
            let Some(Definition::Function(function_definition)) =
                binder.find_definition_by_id(*definition_id)
            else {
                return None;
            };
            function_definition.externalized_type_id?
        }
        _ => type_id,
    };
    match types.get_type_by_id(type_id) {
        Type::Function(FunctionType {
            parameter_types,
            call_mode,
            ..
        }) if call_mode.is_external() => Some(parameter_types),
        _ => None,
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

/// The parameters declared by `definition_id`, if it declares any, eg. a
/// function. A getter declares none, and neither does a struct, whose scope
/// holds its members.
// TODO: replace this with `Binder::get_parameters_for_definition` once #2370
// is merged.
pub(super) fn declared_parameters(
    binder: &Binder,
    definition_id: Option<NodeId>,
) -> Option<&[ParameterDefinition]> {
    let parameters_scope_id = binder.get_parameters_scope_for_definition(definition_id?)?;
    match binder.get_scope_by_id(parameters_scope_id) {
        Scope::Parameters(parameters_scope) => Some(&parameters_scope.parameters),
        _ => None,
    }
}

/// The `parameters` a call passes arguments for. A bound call, eg. `x.f(..)`,
/// already has its first argument, so a named argument cannot target it.
fn without_bound_receiver<T>(parameters: &[T], call_mode: CallMode) -> &[T] {
    let skip = usize::from(call_mode.is_bound());
    parameters.get(skip..).unwrap_or_default()
}

/// What creating a value with `new` takes.
enum CreationParameters<'a> {
    /// The parameters of a contract's constructor, or none if it declares no
    /// constructor.
    Constructor(&'a [ParameterDefinition]),
    /// The length of a dynamic array, `bytes` or `string`.
    Length(TypeId),
}

/// What creating a value of `created_type_id` with `new` takes, if it can be
/// created at all.
fn creation_parameters<'a>(
    binder: &'a Binder,
    types: &TypeRegistry,
    created_type_id: TypeId,
) -> Option<CreationParameters<'a>> {
    match types.get_type_by_id(created_type_id) {
        Type::Contract(ContractType { definition_id }) => {
            // Interfaces and libraries cannot be created.
            let Some(Definition::Contract(_)) = binder.find_definition_by_id(*definition_id) else {
                return None;
            };
            let parameters = declared_parameters(binder, Some(*definition_id)).unwrap_or_default();
            Some(CreationParameters::Constructor(parameters))
        }
        Type::Array(_) | Type::Bytes(_) | Type::String(_) => {
            Some(CreationParameters::Length(types.uint256()))
        }
        _ => None,
    }
}

/// The parameters of the constructor of the base `definition_id`, if it is a
/// contract or an interface. Only the base's own constructor counts: without
/// one, the base takes no arguments, even if it inherits a constructor that
/// does. Surprisingly, an interface can also be invoked as a base constructor,
/// and like a contract without a constructor, it takes no arguments.
fn base_constructor_parameters(
    binder: &Binder,
    definition_id: NodeId,
) -> Option<&[ParameterDefinition]> {
    match binder.find_definition_by_id(definition_id)? {
        Definition::Contract(_) | Definition::Interface(_) => {
            Some(declared_parameters(binder, Some(definition_id)).unwrap_or_default())
        }
        _ => None,
    }
}
