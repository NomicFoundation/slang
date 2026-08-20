use super::fixtures;
use crate::define_fixture;

#[test]
fn test_compute_internal_signature() {
    let unit = super::FullAbi::build_compilation_unit();

    let test_contract = unit
        .find_contract_by_name("Test")
        .next()
        .expect("Test contract can be found");

    // Functions are linearised in alphabetical order, with the unnamed
    // `receive`/`fallback` functions sorted first.
    let functions = test_contract.linearised_functions();
    assert_eq!(functions.len(), 3);

    assert_eq!(
        functions[0].compute_internal_signature(),
        Some("receive()".to_string())
    );
    assert_eq!(
        functions[1].compute_internal_signature(),
        Some("fallback()".to_string())
    );
    assert_eq!(
        functions[2].compute_internal_signature(),
        Some("foo(uint256)".to_string())
    );

    // The constructor is not part of the linearised functions list.
    let constructor = test_contract
        .constructor()
        .expect("Test contract has a constructor");
    assert_eq!(
        constructor.compute_internal_signature(),
        Some("@constructor()".to_string())
    );

    // State variables are linearised in storage layout order (bases first).
    // Only the public variables defines a getter with an internal signature.
    let state_variables = test_contract.linearised_state_variables();
    assert_eq!(state_variables.len(), 3);
    assert_eq!(
        state_variables[0].compute_internal_signature(),
        Some("xs(uint256)".to_string())
    );
    assert_eq!(state_variables[1].compute_internal_signature(), None,);
    assert_eq!(
        state_variables[2].compute_internal_signature(),
        Some("b()".to_string())
    );

    // Errors and events are linearised with base contracts first.
    let errors = test_contract.linearised_errors();
    assert_eq!(errors.len(), 2);
    assert_eq!(
        errors[0].compute_internal_signature(),
        Some("SomethingWrong(string)".to_string())
    );
    assert_eq!(
        errors[1].compute_internal_signature(),
        Some("InsufficientBalance(uint256,uint256)".to_string())
    );

    let events = test_contract.linearised_events();
    assert_eq!(events.len(), 2);
    assert_eq!(
        events[0].compute_internal_signature(),
        Some("BaseEvent(uint256,string)".to_string())
    );
    assert_eq!(
        events[1].compute_internal_signature(),
        Some("Event(uint256,bytes32)".to_string())
    );
}

define_fixture!(
    NestedKeyGetter,
    file: "main.sol", r#"
contract C {
    enum E { A }

    mapping(E => uint256) public m;
}
"#,
);

#[test]
fn test_getter_internal_signature_qualifies_a_nested_key() {
    let unit = NestedKeyGetter::build_compilation_unit();
    let contract = unit
        .find_contract_by_name("C")
        .next()
        .expect("contract can be found");
    let state_variables = contract.state_variables();

    assert_eq!(
        state_variables[0].compute_internal_signature(),
        Some("m(C.E)".to_string())
    );
}

#[test]
fn test_compute_internal_signature_of_library_member() {
    let unit = super::LibraryAbi::build_compilation_unit();
    let functions = fixtures::find_library(&unit, "L").functions();

    // A type declared in the library is named under its enclosing scope.
    assert_eq!(
        functions[0].compute_internal_signature(),
        Some("f(L.S,uint256)".to_string())
    );
}

#[test]
fn test_compute_library_signature() {
    let unit = super::LibraryAbi::build_compilation_unit();
    let functions = fixtures::find_library(&unit, "L").functions();
    let [struct_storage, array_storage, _, _, user_defined_value] = functions.as_slice() else {
        panic!("expected the library to declare five functions");
    };

    // The library form spells the data location the internal form omits...
    assert_eq!(
        struct_storage.compute_library_signature(),
        Some("f(L.S storage,uint256)".to_string())
    );
    assert_eq!(
        array_storage.compute_library_signature(),
        Some("g(uint256[] storage)".to_string())
    );

    // ... and unwraps a user-defined value type to its underlying type.
    assert_eq!(
        user_defined_value.compute_library_signature(),
        Some("j(uint64)".to_string())
    );
    assert_eq!(
        user_defined_value.compute_internal_signature(),
        Some("j(L.U)".to_string())
    );
}

#[test]
fn test_library_signature_unwraps_a_user_defined_value_type_under_an_array() {
    let unit = super::LibraryUdvtAbi::build_compilation_unit();
    let functions = fixtures::find_library(&unit, "L").functions();
    let [array, fixed_size_array, mapping] = functions.as_slice() else {
        panic!("expected the library to declare three functions");
    };

    // An array spells its element as the wrapped type, while a mapping keeps
    // the wrapper's name (matching solc, which has no ABI spelling for one).
    assert_eq!(
        array.compute_library_signature(),
        Some("k(uint64[])".to_string())
    );
    assert_eq!(
        fixed_size_array.compute_library_signature(),
        Some("q(uint64[2])".to_string())
    );
    assert_eq!(
        mapping.compute_library_signature(),
        Some("m(mapping(uint256 => L.U) storage)".to_string())
    );
}

define_fixture!(
    AliasedLibraryType,
    file: "main.sol", r#"
import {L as M} from "library.sol";

library U {
    function f(M.S storage s) external view returns (uint256) { return s.a; }
}
"#,
    file: "library.sol", r#"
library L {
    struct S { uint256 a; }
}
"#,
);

#[test]
fn test_library_signature_names_an_aliased_type_at_its_declaration() {
    let unit = AliasedLibraryType::build_compilation_unit();
    let functions = fixtures::find_library(&unit, "U").functions();

    assert_eq!(
        functions[0].compute_library_signature(),
        Some("f(L.S storage)".to_string())
    );
}
