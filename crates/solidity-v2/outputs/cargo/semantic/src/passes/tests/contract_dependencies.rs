//! Tests for the per-phase contract bytecode dependency maps and the
//! merged view derived from them.
//!
//! The last section pins the walk orders where slang records a different
//! expression than solc for a dependency they both find.

use slang_solidity_v2_common::diagnostics::kinds::DiagnosticKind;
use slang_solidity_v2_common::diagnostics::kinds::semantic::CyclicBytecodeDependency;
use slang_solidity_v2_common::evm_targets::EvmTarget;
use slang_solidity_v2_common::nodes::NodeId;

use super::{Analyse, Analysis, AnalysisBuilder};
use crate::binder::Definition;
use crate::context::SemanticContext;

/// These tests pin Istanbul: the dependency walk is target independent, and
/// the oldest supported target keeps any target-gated built-in out of the way.
const TARGET: EvmTarget = EvmTarget::Istanbul;

fn build_context(source: &str) -> SemanticContext {
    analyse(source).run().expect_no_diagnostics().into_context()
}

/// Runs the pipeline without asserting on the diagnostics, for sources whose
/// dependencies form a cycle. The whole [`Analysis`] comes back so the test
/// can reach both the context and what was reported building it.
fn analyse(source: &str) -> AnalysisBuilder<'_> {
    Analysis::of_source(source)
        .target(TARGET)
        .analyse(Analyse::Context)
}

fn contract_id(context: &SemanticContext, name: &str) -> NodeId {
    context
        .find_contract_by_name(name)
        .next()
        .expect("contract exists")
        .id()
}

fn library_id(context: &SemanticContext, name: &str) -> NodeId {
    *context
        .binder()
        .definitions()
        .iter()
        .find(|(_, definition)| {
            matches!(definition, Definition::Library(library)
                if library.ir_node.name.unparse() == name)
        })
        .expect("library exists")
        .0
}

#[test]
fn new_expression_records_the_dependency() {
    let source = "contract C { constructor() { new D(); } }
        contract D {}";
    let context = build_context(source);

    let c = contract_id(&context, "C");
    let d = contract_id(&context, "D");

    let dependencies = &context.contract_dependencies()[&c];
    assert_eq!(vec![d], dependencies.keys().copied().collect::<Vec<_>>());
    assert_eq!("new D", &source[dependencies[&d].range()]);
}

#[test]
fn the_first_reference_wins() {
    // The initializer runs at creation, which is traversed before the
    // deployed entry points, so its expression is the recorded one.
    let source = "contract C {
            D d = new D();
            function f() public { new D(); }
        }
        contract D {}";
    let context = build_context(source);

    let c = contract_id(&context, "C");
    let d = contract_id(&context, "D");

    let reference = &context.contract_dependencies()[&c][&d];
    let first = source.find("new D").expect("reference exists");
    assert_eq!(first, reference.range().start);
}

#[test]
fn definitions_without_dependencies_have_no_entry() {
    let source = "contract A { function f() public { new B(); } }
        contract B {}";
    let context = build_context(source);

    let a = contract_id(&context, "A");
    let b = contract_id(&context, "B");
    assert_eq!(
        vec![a],
        context
            .contract_dependencies()
            .keys()
            .copied()
            .collect::<Vec<_>>()
    );
    assert!(!context.contract_dependencies().contains_key(&b));
}

#[test]
fn programs_without_contract_references_have_no_dependencies() {
    let source = "contract A { function f() public {} }
        contract B is A {}";
    let context = build_context(source);

    assert!(context.contract_dependencies().is_empty());
    assert!(context.creation_bytecode_dependencies().is_empty());
    assert!(context.deployed_bytecode_dependencies().is_empty());
}

#[test]
fn constructor_dependency_is_creation_only() {
    let source = "contract C { constructor() { new D(); } }
        contract D {}";
    let context = build_context(source);

    let c = contract_id(&context, "C");
    let d = contract_id(&context, "D");

    assert!(context.creation_bytecode_dependencies()[&c].contains_key(&d));
    assert!(!context.deployed_bytecode_dependencies().contains_key(&c));
}

#[test]
fn public_function_dependency_is_deployed_only() {
    let source = "contract C { function f() public { new D(); } }
        contract D {}";
    let context = build_context(source);

    let c = contract_id(&context, "C");
    let d = contract_id(&context, "D");

    assert!(!context.creation_bytecode_dependencies().contains_key(&c));
    assert!(context.deployed_bytecode_dependencies()[&c].contains_key(&d));
}

#[test]
fn an_abstract_contract_has_dependencies() {
    // An abstract contract is never deployed, but its code is still walked.
    // Its initializer runs at creation and its entry points after deployment,
    // exactly as for any other contract.
    let source = "abstract contract A {
            D d = new D();
            function f() public { new E(); }
            function g() public virtual;
        }
        contract D {}
        contract E {}";
    let context = build_context(source);

    let a = contract_id(&context, "A");
    let d = contract_id(&context, "D");
    let e = contract_id(&context, "E");

    assert!(context.creation_bytecode_dependencies()[&a].contains_key(&d));
    assert!(context.deployed_bytecode_dependencies()[&a].contains_key(&e));
}

#[test]
fn dependency_reachable_from_both_phases_is_in_both_maps() {
    let source = "contract C {
            D d = new D();
            function f() public { new D(); }
        }
        contract D {}";
    let context = build_context(source);

    let c = contract_id(&context, "C");
    let d = contract_id(&context, "D");

    let first = source.find("new D").expect("reference exists");
    let second = source.rfind("new D").expect("reference exists");

    let creation = &context.creation_bytecode_dependencies()[&c][&d];
    assert_eq!(first, creation.range().start);
    let deployed = &context.deployed_bytecode_dependencies()[&c][&d];
    assert_eq!(second, deployed.range().start);

    // The merged map prefers the creation entry.
    let merged = &context.contract_dependencies()[&c];
    assert_eq!(1, merged.len());
    assert_eq!(first, merged[&d].range().start);
}

#[test]
fn function_value_taken_at_creation_seeds_the_deployed_walk() {
    // The deployed code never names `helper`, but its pointer is stored
    // during creation and can be dispatched after deployment.
    let source = "function helper() { new B(); }
        contract A {
            function() internal ptr;
            constructor() { ptr = helper; }
            function run() public { ptr(); }
        }
        contract B {}";
    let context = build_context(source);

    let a = contract_id(&context, "A");
    let b = contract_id(&context, "B");

    assert!(context.creation_bytecode_dependencies()[&a].contains_key(&b));
    assert!(context.deployed_bytecode_dependencies()[&a].contains_key(&b));
}

#[test]
fn function_value_taken_after_deployment_stays_out_of_the_creation_map() {
    let source = "function helper() { new B(); }
        contract A {
            function() internal ptr;
            function run() public { ptr = helper; ptr(); }
        }
        contract B {}";
    let context = build_context(source);

    let a = contract_id(&context, "A");
    let b = contract_id(&context, "B");

    assert!(!context.creation_bytecode_dependencies().contains_key(&a));
    assert!(context.deployed_bytecode_dependencies()[&a].contains_key(&b));
}

#[test]
fn qualified_modifier_invocation_runs_the_named_modifier() {
    // `A.m` runs A's modifier even though C overrides it.
    let source = "contract A {
            modifier m() virtual { new B(); _; }
        }
        contract C is A {
            modifier m() override { _; }
            function f() public A.m {}
        }
        contract B {}";
    let context = build_context(source);

    let c = contract_id(&context, "C");
    let b = contract_id(&context, "B");

    assert!(context.deployed_bytecode_dependencies()[&c].contains_key(&b));
}

#[test]
fn qualified_modifier_invocation_skips_the_override() {
    // The override never runs under a qualified invocation, so the
    // `new B()` inside it is unreachable.
    let source = "contract A {
            modifier m() virtual { _; }
        }
        contract C is A {
            modifier m() override { new B(); _; }
            function f() public A.m {}
        }
        contract B {}";
    let context = build_context(source);

    assert!(context.contract_dependencies().is_empty());
}

#[test]
fn initializers_run_before_the_constructor() {
    // The constructor is declared first, but initializers run first, so
    // the initializer's expression is the recorded one.
    let source = "contract C {
            constructor() { new D(); }
            D d = new D();
        }
        contract D {}";
    let context = build_context(source);

    let c = contract_id(&context, "C");
    let d = contract_id(&context, "D");

    let reference = &context.contract_dependencies()[&c][&d];
    let last = source.rfind("new D").expect("reference exists");
    assert_eq!(last, reference.range().start);
}

#[test]
fn virtual_call_resolves_against_the_most_derived_contract() {
    // A runs its own f. C runs the override, which has no reference.
    let source = "contract A {
            function f() public virtual { new B(); }
            function g() public { f(); }
        }
        contract C is A {
            function f() public override {}
        }
        contract B {}";
    let context = build_context(source);

    let a = contract_id(&context, "A");
    let c = contract_id(&context, "C");
    let b = contract_id(&context, "B");

    assert!(context.deployed_bytecode_dependencies()[&a].contains_key(&b));
    assert!(!context.deployed_bytecode_dependencies().contains_key(&c));
}

#[test]
fn super_call_skips_bodiless_declarations() {
    // C.f has no body, so `super.f()` in D runs B.f.
    let source = "abstract contract B {
            function f() public virtual { new E(); }
        }
        abstract contract C {
            function f() public virtual;
        }
        contract D is B, C {
            function f() public override(B, C) { super.f(); }
        }
        contract E {}";
    let context = build_context(source);

    let d = contract_id(&context, "D");
    let e = contract_id(&context, "E");

    assert!(context.deployed_bytecode_dependencies()[&d].contains_key(&e));
}

#[test]
fn internal_library_call_embeds_into_the_caller() {
    let source = "library L {
            function g() internal { new B(); }
        }
        contract A {
            function f() public { L.g(); }
        }
        contract B {}";
    let context = build_context(source);

    let a = contract_id(&context, "A");
    let b = contract_id(&context, "B");

    assert!(context.deployed_bytecode_dependencies()[&a].contains_key(&b));
}

#[test]
fn public_library_call_does_not_embed() {
    // The call is a delegatecall into the deployed library, so only the
    // library itself embeds B.
    let source = "library L {
            function g() public { new B(); }
        }
        contract A {
            function f() public { L.g(); }
        }
        contract B {}";
    let context = build_context(source);

    let a = contract_id(&context, "A");
    let l = library_id(&context, "L");
    let b = contract_id(&context, "B");

    assert!(!context.deployed_bytecode_dependencies().contains_key(&a));
    assert!(context.deployed_bytecode_dependencies()[&l].contains_key(&b));
}

#[test]
fn attached_function_on_a_contract_value_embeds_into_the_caller() {
    let source = "library L {
            function g(C self) internal { new B(); }
        }
        contract C {
            using L for C;
            function f(C other) public { other.g(); }
        }
        contract B {}";
    let context = build_context(source);

    let c = contract_id(&context, "C");
    let b = contract_id(&context, "B");

    assert!(context.deployed_bytecode_dependencies()[&c].contains_key(&b));
}

#[test]
fn attached_free_function_on_a_contract_value_is_followed() {
    let source = "function g(C self) { new B(); }
        contract C {
            using {g} for C;
            function f(C other) public { other.g(); }
        }
        contract B {}";
    let context = build_context(source);

    let c = contract_id(&context, "C");
    let b = contract_id(&context, "B");

    assert!(context.deployed_bytecode_dependencies()[&c].contains_key(&b));
}

#[test]
fn attached_function_on_an_interface_value_is_followed() {
    let source = "interface I {}
        library L {
            function g(I self) internal { new B(); }
        }
        contract A {
            using L for I;
            function f(I x) public { x.g(); }
        }
        contract B {}";
    let context = build_context(source);

    let a = contract_id(&context, "A");
    let b = contract_id(&context, "B");

    assert!(context.deployed_bytecode_dependencies()[&a].contains_key(&b));
}

#[test]
fn code_access_records_a_deployed_dependency_for_the_library() {
    let source = "library L {
            function f() public pure returns (bytes memory) {
                return type(L).creationCode;
            }
        }";
    let analysis = analyse(source).run();
    let context = analysis.context();
    // The self dependency is reported as a cycle at the code access.
    let diagnostics: Vec<_> = analysis.diagnostics.iter().collect();
    let [diagnostic] = diagnostics[..] else {
        panic!("Expected one diagnostic: {diagnostics:?}");
    };
    assert_eq!(
        DiagnosticKind::from(CyclicBytecodeDependency),
        *diagnostic.kind()
    );
    assert_eq!(
        "type(L).creationCode",
        &source[diagnostic.text_range().clone()]
    );

    let l = library_id(context, "L");

    let reference = &context.deployed_bytecode_dependencies()[&l][&l];
    assert_eq!("type(L).creationCode", &source[reference.range()]);
    assert!(context.creation_bytecode_dependencies().is_empty());
}

#[test]
fn constant_is_followed_through_bare_name_and_member_access() {
    // E reads its own constant by bare name, A reads the library's constant
    // as a member. Both compile the value in and depend on D. The library
    // itself does not, since nothing of its own reaches the constant.
    let source = "library B {
            bytes constant CODE = type(D).creationCode;
        }
        contract A {
            function f() public pure returns (bytes memory) { return B.CODE; }
        }
        contract E {
            bytes constant CODE = type(D).creationCode;
            function g() public pure returns (bytes memory) { return CODE; }
        }
        contract D {}";
    let context = build_context(source);

    let a = contract_id(&context, "A");
    let e = contract_id(&context, "E");
    let d = contract_id(&context, "D");

    let dependencies = context.contract_dependencies();
    assert_eq!(2, dependencies.len());
    assert!(dependencies[&a].contains_key(&d));
    assert!(dependencies[&e].contains_key(&d));
}

#[test]
fn a_constant_embedding_the_reader_records_a_self_dependency() {
    // The library constant's value compiles into f, so A's deployed code
    // embeds A's own creation code.
    let source = "library B {
            bytes constant CODE = type(A).creationCode;
        }
        contract A {
            function f() public pure returns (bytes memory) { return B.CODE; }
        }";
    let analysis = analyse(source).run();
    let context = analysis.context();
    // The self dependency is reported as a cycle at the code access.
    let diagnostics: Vec<_> = analysis.diagnostics.iter().collect();
    let [diagnostic] = diagnostics[..] else {
        panic!("Expected one diagnostic: {diagnostics:?}");
    };
    assert_eq!(
        DiagnosticKind::from(CyclicBytecodeDependency),
        *diagnostic.kind()
    );
    assert_eq!(
        "type(A).creationCode",
        &source[diagnostic.text_range().clone()]
    );

    let a = contract_id(context, "A");

    let reference = &context.deployed_bytecode_dependencies()[&a][&a];
    assert_eq!("type(A).creationCode", &source[reference.range()]);
}

#[test]
fn a_base_constant_is_followed_through_the_base_name() {
    // The constant has no getter, so nothing seeds it. Derived reads it
    // through the base name, which compiles the value in.
    let source = "contract Base {
            bytes internal constant CODE = type(D).creationCode;
        }
        contract Derived is Base {
            function f() public pure returns (bytes memory) {
                return Base.CODE;
            }
        }
        contract D {}";
    let context = build_context(source);

    let derived = contract_id(&context, "Derived");
    let d = contract_id(&context, "D");

    let dependencies = context.contract_dependencies();
    assert_eq!(1, dependencies.len());
    assert!(dependencies[&derived].contains_key(&d));
}

#[test]
fn a_public_constant_is_followed_through_the_type_name() {
    // Reading the constant through the library name compiles the value
    // into f.
    let source = "library B {
            bytes public constant CODE = type(D).creationCode;
        }
        contract A {
            function f() public pure returns (bytes memory) { return B.CODE; }
        }
        contract D {}";
    let context = build_context(source);

    let a = contract_id(&context, "A");
    let d = contract_id(&context, "D");

    assert!(context.contract_dependencies()[&a].contains_key(&d));
}

#[test]
fn a_storage_variable_read_through_the_base_name_is_not_followed() {
    // Base.x reads storage. The initializer runs at creation and is not
    // part of g's code, so the dependency stays in the creation map only.
    let source = "contract Base {
            bytes x = type(D).creationCode;
        }
        contract Derived is Base {
            function g() public view returns (bytes memory) {
                return Base.x;
            }
        }
        contract D {}";
    let context = build_context(source);

    let base = contract_id(&context, "Base");
    let derived = contract_id(&context, "Derived");
    let d = contract_id(&context, "D");

    assert!(context.creation_bytecode_dependencies()[&base].contains_key(&d));
    assert!(context.creation_bytecode_dependencies()[&derived].contains_key(&d));
    assert!(context.deployed_bytecode_dependencies().is_empty());
}

#[test]
fn a_getter_call_on_a_contract_value_is_not_followed() {
    // The getter body runs in B's deployed code, so B embeds D. A only
    // makes an external call and receives the value at runtime, so its
    // bytecode embeds nothing.
    let source = "contract B {
            bytes public constant CODE = type(D).creationCode;
        }
        contract A {
            function f(B b) public view returns (bytes memory) {
                return b.CODE();
            }
        }
        contract D {}";
    let context = build_context(source);

    let a = contract_id(&context, "A");
    let b = contract_id(&context, "B");
    let d = contract_id(&context, "D");

    let dependencies = context.contract_dependencies();
    assert!(dependencies[&b].contains_key(&d));
    assert!(!dependencies.contains_key(&a));
}

#[test]
fn public_constant_is_a_deployed_entry_point() {
    // Nothing references the constant. Its getter returns the value, so the
    // value is still part of the deployed code.
    let source = "contract B {
            bytes public constant CODE = type(D).creationCode;
        }
        contract D {}";
    let context = build_context(source);

    let b = contract_id(&context, "B");
    let d = contract_id(&context, "D");

    let reference = &context.deployed_bytecode_dependencies()[&b][&d];
    assert_eq!("type(D).creationCode", &source[reference.range()]);
    assert!(context.creation_bytecode_dependencies().is_empty());
}

#[test]
fn constant_without_a_getter_is_not_an_entry_point() {
    let source = "contract B {
            bytes constant CODE = type(D).creationCode;
            bytes internal constant OTHER = type(D).creationCode;
        }
        contract D {}";
    let context = build_context(source);

    assert!(context.contract_dependencies().is_empty());
}

#[test]
fn an_inherited_public_constant_is_an_entry_point() {
    // Both contracts serve the constant's selector.
    let source = "contract Base {
            bytes public constant CODE = type(D).creationCode;
        }
        contract Derived is Base {}
        contract D {}";
    let context = build_context(source);

    let base = contract_id(&context, "Base");
    let derived = contract_id(&context, "Derived");
    let d = contract_id(&context, "D");

    assert!(context.deployed_bytecode_dependencies()[&base].contains_key(&d));
    assert!(context.deployed_bytecode_dependencies()[&derived].contains_key(&d));
}

#[test]
fn a_library_public_constant_is_a_deployed_entry_point() {
    let source = "library L {
            bytes public constant CODE = type(D).creationCode;
        }
        contract D {}";
    let context = build_context(source);

    let l = library_id(&context, "L");
    let d = contract_id(&context, "D");

    let reference = &context.deployed_bytecode_dependencies()[&l][&d];
    assert_eq!("type(D).creationCode", &source[reference.range()]);
}

#[test]
fn a_library_constant_without_a_getter_is_not_an_entry_point() {
    // The value is compiled only into referencing units, and nothing
    // references it.
    let source = "library L {
            bytes internal constant CODE = type(L).creationCode;
        }";
    let context = build_context(source);

    assert!(context.contract_dependencies().is_empty());
}

#[test]
fn a_constant_overriding_a_function_is_an_entry_point() {
    // The constant's getter serves x's selector in B, so its value is part
    // of B's deployed code.
    let source = "abstract contract Base {
            function x() external pure virtual returns (bytes memory);
        }
        contract B is Base {
            bytes public constant override x = type(D).creationCode;
        }
        contract D {}";
    let context = build_context(source);

    let b = contract_id(&context, "B");
    let d = contract_id(&context, "D");

    assert!(context.deployed_bytecode_dependencies()[&b].contains_key(&d));
}

#[test]
fn a_function_overridden_by_a_constant_stays_out() {
    // The getter serves x's selector in B, so Base.x's body is not part of
    // B's deployed code.
    let source = "contract Base {
            function x() external virtual returns (bytes memory) {
                return type(D).creationCode;
            }
        }
        contract B is Base {
            bytes public constant override x = \"\";
        }
        contract D {}";
    let context = build_context(source);

    let base = contract_id(&context, "Base");
    let b = contract_id(&context, "B");
    let d = contract_id(&context, "D");

    let dependencies = context.contract_dependencies();
    assert!(dependencies[&base].contains_key(&d));
    assert!(!dependencies.contains_key(&b));
}

#[test]
fn user_defined_operator_function_is_followed() {
    let source = "type Int is int256;
        function add(Int, Int) pure returns (Int) {
            bytes memory code = type(B).creationCode;
            code;
            return Int.wrap(0);
        }
        using {add as +} for Int global;
        contract A {
            function f(Int x, Int y) public pure returns (Int) { return x + y; }
        }
        contract B {}";
    let context = build_context(source);

    let a = contract_id(&context, "A");
    let b = contract_id(&context, "B");

    assert!(context.deployed_bytecode_dependencies()[&a].contains_key(&b));
}

// The tests below pin the walk orders that make slang record a different
// expression than solc. The dependency is the same in every case, only the
// expression standing for it differs, so a cycle diagnostic lands on another
// spot on the same cycle. Where the two contracts of a hierarchy share the
// expression slang picks, one report is dropped as already reported.

#[test]
fn a_base_constructor_wins_over_a_derived_initializer() {
    // Every creation unit joins one queue, so Base's constructor is walked
    // before Mid's initializer. solc visits every base's initializers before
    // any constructor body and records Mid's own initializer instead.
    let source = "contract Base { constructor() { new D(); } }
        contract Mid is Base { D d = new D(); }
        contract D {}";
    let context = build_context(source);

    let mid = contract_id(&context, "Mid");
    let d = contract_id(&context, "D");

    let reference = &context.contract_dependencies()[&mid][&d];
    let in_base_constructor = source.find("new D").expect("reference exists");
    assert_eq!(in_base_constructor, reference.range().start);
}

#[test]
fn an_entry_point_earlier_by_name_wins() {
    // Deployed entry points come from the linearised function list, which is
    // sorted by name, so `alpha` is walked before `zebra`. solc walks the
    // external interface in declaration order and records `zebra`'s `new B`.
    let source = "contract A {
            function zebra() public { new B(); }
            function alpha() public pure returns (bytes memory) { return type(B).creationCode; }
        }
        contract B {}";
    let context = build_context(source);

    let a = contract_id(&context, "A");
    let b = contract_id(&context, "B");

    let reference = &context.contract_dependencies()[&a][&b];
    assert_eq!("type(B).creationCode", &source[reference.range()]);
}

#[test]
fn an_unnamed_entry_point_wins_over_a_named_one() {
    // The linearised function list puts the unnamed fallback and receive
    // before the named functions. solc walks the external interface first
    // and only then the fallback, so it records `f`'s code access.
    let source = "contract A {
            fallback() external { new B(); }
            function f() public pure returns (bytes memory) { return type(B).creationCode; }
        }
        contract B {}";
    let context = build_context(source);

    let a = contract_id(&context, "A");
    let b = contract_id(&context, "B");

    let reference = &context.contract_dependencies()[&a][&b];
    assert_eq!("new B", &source[reference.range()]);
}

#[test]
fn a_units_own_reference_wins_over_the_constant_it_uses() {
    // A unit's own references are recorded before the constants it uses are
    // walked. solc compiles a constant in where it is used, so it records
    // the access inside `K` for both.
    let source = "bytes constant K = type(B).creationCode;
        contract A {
            function f() public pure returns (bytes memory) {
                bytes memory value = K;
                value;
                return type(B).creationCode;
            }
        }
        contract B {}";
    let context = build_context(source);

    let a = contract_id(&context, "A");
    let b = contract_id(&context, "B");

    let reference = &context.contract_dependencies()[&a][&b];
    let in_function = source
        .rfind("type(B).creationCode")
        .expect("reference exists");
    assert_eq!(in_function, reference.range().start);
}
