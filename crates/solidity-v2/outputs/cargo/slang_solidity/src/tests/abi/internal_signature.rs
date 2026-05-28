#[test]
fn test_compute_internal_signature() {
    let unit = super::FullAbi::build_compilation_unit();

    let test_contract = unit
        .find_contract_by_name("Test")
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
