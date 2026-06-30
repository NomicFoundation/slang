use super::fixtures;

#[test]
fn test_function_selector() {
    let unit = fixtures::Counter::build_compilation_unit();

    let counter = unit
        .find_contract_by_name("Counter")
        .next()
        .expect("contract can be found");

    let functions = counter.linearised_functions();
    assert_eq!(functions.len(), 5);

    // all the functions in the contract are public
    assert_eq!(functions[0].compute_selector(), Some(0x7d55_923d_u32)); // click()
    assert_eq!(functions[1].compute_selector(), Some(0x2f27_70db_u32)); // disable()
    assert_eq!(functions[2].compute_selector(), Some(0xa390_7d71_u32)); // enable()
    assert_eq!(functions[3].compute_selector(), Some(0x7cf5_dab0_u32)); // increment(uint256)
    assert_eq!(functions[4].compute_selector(), Some(0x6aa6_33b6_u32)); // isEnabled()

    let state_variables = counter.linearised_state_variables();
    assert_eq!(state_variables.len(), 4);

    // for state variables, selectors only make sense for public getters
    assert_eq!(state_variables[0].compute_selector(), None); // _owner
    assert_eq!(state_variables[1].compute_selector(), None); // _state
    assert_eq!(state_variables[2].compute_selector(), Some(0x0666_1abd_u32)); // count()
    assert_eq!(state_variables[3].compute_selector(), None); // _clickers
}

#[test]
fn test_events_and_errors_selectors() {
    let unit = super::FullAbi::build_compilation_unit();

    let test_contract = unit
        .find_contract_by_name("Test")
        .next()
        .expect("Test contract can be found");

    let events = test_contract.events();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].compute_selector(), Some(0xb9b1_0fa6)); // Event(uint256,bytes32)
    assert_eq!(
        events[0].compute_topic(),
        Some([
            0xb9, 0xb1, 0x0f, 0xa6, 0x33, 0x03, 0x36, 0xbe, 0xe8, 0x83, 0x55, 0x7e, 0x90, 0x6a,
            0xb0, 0xd5, 0xe9, 0x8e, 0xe5, 0x03, 0x06, 0x9e, 0x9c, 0x49, 0x68, 0x9f, 0x95, 0x02,
            0x2d, 0xb8, 0x13, 0x99,
        ])
    );

    let errors = test_contract.errors();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].compute_selector(), Some(0xcf47_9181)); // InsufficientBalance(uint256,uint256)
}

#[test]
fn test_selectors_for_functions_with_tuple_parameters() {
    let unit = super::AbiWithTuples::build_compilation_unit();

    let test = unit
        .find_contract_by_name("Test")
        .next()
        .expect("contract is found");
    let functions = test.functions();

    // f((uint256,uint256[],(uint256,uint256)[]),(uint256,uint256),uint256)
    assert_eq!(functions[0].compute_selector(), Some(0x6f2b_e728_u32));
    // g()
    assert_eq!(functions[1].compute_selector(), Some(0xe217_9b8e_u32));
}
