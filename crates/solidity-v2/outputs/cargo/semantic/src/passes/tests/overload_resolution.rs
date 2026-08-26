use slang_solidity_v2_common::diagnostics::kinds::resolution::AmbiguousReference;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_ir::ir;

use super::{Analyse, Analysis, diagnostic_kind};
use crate::binder::{Binder, Resolution};
use crate::types::{
    ContractType, FunctionType, FunctionTypeMutability, FunctionTypeVisibility, Type, TypeId,
    TypeRegistry,
};

/// The resolutions of every reference made through an identifier named `name`.
fn resolutions_of(binder: &Binder, name: &str) -> Vec<Resolution> {
    binder
        .references()
        .values()
        .filter(|reference| reference.identifier.unparse() == name)
        .map(|reference| reference.resolution.clone())
        .collect()
}

/// The node ids of `contract`'s function definitions named `name`, in
/// declaration order.
fn function_definition_ids(contract: &ir::ContractDefinition, name: &str) -> Vec<NodeId> {
    contract
        .members
        .iter()
        .filter_map(|member| match member {
            ir::ContractMember::FunctionDefinition(function)
                if function.name.as_ref().is_some_and(|n| n.unparse() == name) =>
            {
                Some(function.id())
            }
            _ => None,
        })
        .collect()
}

/// Two unrelated functions attached to the same type under one name by
/// different `using` directives are genuinely ambiguous. They have identical
/// signatures, but neither is a contract member, so neither can override the
/// other.
#[test]
fn test_attached_functions_from_different_directives_stay_ambiguous() {
    const SOURCE: &str = r###"
struct S {
    uint256 x;
}

function f(S memory s) pure returns (uint256) {
    return s.x;
}

library L {
    function f(S memory s) internal pure returns (uint256) {
        return s.x + 1;
    }
}

contract C {
    using {f} for S;
    using L for S;

    function test(S memory s) internal pure {
        s.f;
    }
}
    "###;

    // `s.f` names the overload set as a value, which is itself reported; the
    // resolution behind it is what this test is about.
    let analysis = Analysis::of_source(SOURCE).run(Analyse::References);
    assert_eq!(
        Some(
            AmbiguousReference {
                name: "f".to_owned()
            }
            .into()
        ),
        diagnostic_kind(&analysis.diagnostics),
    );

    // The `f` in the `using {f} for S` clause resolves to the free function,
    // and the `f` in `s.f` sees both attached candidates.
    let ambiguous = resolutions_of(analysis.binder(), "f")
        .into_iter()
        .filter_map(|resolution| match resolution {
            Resolution::Ambiguous(definition_ids) => Some(definition_ids),
            _ => None,
        })
        .collect::<Vec<_>>();

    assert_eq!(
        1,
        ambiguous.len(),
        "expected exactly one ambiguous reference to `f`"
    );
    let definition_ids = &ambiguous[0];
    assert_eq!(
        2,
        definition_ids.len(),
        "expected both attached functions as candidates"
    );
    assert_ne!(definition_ids[0], definition_ids[1]);
}

/// Overriding is a contract-member relationship: two functions without a
/// receiver never override each other, however identical their signatures.
///
/// Members do *not* need to be declared in contracts that derive from one
/// another. A resolution gathers members from one linearisation and yields them
/// most-derived first, so a sibling base's declaration is shadowed just as a
/// base's is; `super.f()` in a contract whose two bases both declare `f` relies
/// on it.
#[test]
fn test_override_is_a_contract_member_relationship() {
    let mut types = TypeRegistry::new(LanguageVersion::LATEST);
    let return_type = types.void();

    let mut contract_type_of = |definition_id: usize| {
        types.register_type(Type::Contract(ContractType {
            definition_id: definition_id.into(),
        }))
    };
    let base = contract_type_of(1);
    let derived = contract_type_of(2);
    let sibling = contract_type_of(3);
    types.register_super_types(derived, &[derived, base]);

    let function_declared_in = |receiver: Option<TypeId>| FunctionType {
        definition_id: None,
        parameter_types: Vec::new(),
        return_type,
        visibility: FunctionTypeVisibility::Internal,
        mutability: FunctionTypeMutability::NonPayable,
        implicit_receiver_type: receiver,
        partially_applied: false,
    };
    let in_base = function_declared_in(Some(base));
    let in_derived = function_declared_in(Some(derived));
    let in_sibling = function_declared_in(Some(sibling));
    let free = function_declared_in(None);

    // All of these share a signature, so the plain check accepts every pair.
    assert!(types.function_type_overrides(&in_derived, &in_base));
    assert!(types.function_type_overrides(&free, &free));

    // Contract members override, whether related by inheritance or not.
    assert!(types.function_type_overrides_in_hierarchy(&in_derived, &in_base));
    assert!(types.function_type_overrides_in_hierarchy(&in_base, &in_derived));
    assert!(types.function_type_overrides_in_hierarchy(&in_sibling, &in_base));

    // Anything without a receiver does not.
    assert!(!types.function_type_overrides_in_hierarchy(&free, &free));
    assert!(!types.function_type_overrides_in_hierarchy(&free, &in_base));
    assert!(!types.function_type_overrides_in_hierarchy(&in_base, &free));
}

/// A function value carrying `Public` visibility is a plain reference, which
/// denotes the internal function: it converts to an internal function type but
/// not to an external one. Reaching a public function externally externalises
/// it at the point of access, so it no longer carries `Public` here.
#[test]
fn test_public_function_type_does_not_convert_to_external() {
    let mut types = TypeRegistry::new(LanguageVersion::LATEST);
    let return_type = types.void();

    let mut function_type_with = |visibility| {
        types.register_type(Type::Function(FunctionType {
            definition_id: None,
            parameter_types: Vec::new(),
            return_type,
            visibility,
            mutability: FunctionTypeMutability::NonPayable,
            implicit_receiver_type: None,
            partially_applied: false,
        }))
    };
    let public = function_type_with(FunctionTypeVisibility::Public);
    let internal = function_type_with(FunctionTypeVisibility::Internal);
    let external = function_type_with(FunctionTypeVisibility::External);

    assert!(types.implicitly_convertible_to(public, public));
    assert!(types.implicitly_convertible_to(public, internal));
    assert!(!types.implicitly_convertible_to(public, external));

    // The other visibilities are unaffected.
    assert!(types.implicitly_convertible_to(internal, internal));
    assert!(!types.implicitly_convertible_to(internal, external));
    assert!(types.implicitly_convertible_to(external, external));
    assert!(!types.implicitly_convertible_to(external, internal));
}

/// The consequence of the rule above for overload selection: passing a public
/// function as a value picks the overload taking an *internal* function, even
/// though the one taking an external function is declared first and would win
/// if the conversion were allowed.
#[test]
fn test_public_function_argument_selects_internal_overload() {
    const SOURCE: &str = r###"
contract C {
    function target(bytes memory) public {}

    function callback(function(bytes memory) external) private pure {}

    function callback(function(bytes memory) internal) private pure {}

    function test() internal pure {
        callback(target);
    }
}
    "###;

    let analysis = Analysis::of_source(SOURCE)
        .run(Analyse::References)
        .expect_no_diagnostics();

    let contract = analysis.find_contract("C");
    let callbacks = function_definition_ids(contract, "callback");
    assert_eq!(2, callbacks.len(), "expected both `callback` overloads");

    let resolutions = resolutions_of(analysis.binder(), "callback");
    assert_eq!(
        vec![Resolution::Definition(callbacks[1])],
        resolutions,
        "expected the overload taking an internal function"
    );
}
