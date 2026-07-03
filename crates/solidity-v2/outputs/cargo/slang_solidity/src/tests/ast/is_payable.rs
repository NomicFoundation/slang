use crate::define_fixture;

define_fixture!(
    Payable,
    file: "main.sol", r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

contract WithReceive {
    receive() external payable {}
}

contract WithPayableFallback {
    fallback() external payable {}
}

contract WithPlainFallback {
    fallback() external {}
}

contract WithNeither {
    function foo() external {}
}

contract InheritsPayable is WithReceive {}
"#,
);

fn is_payable(contract_name: &str) -> bool {
    let unit = Payable::build_compilation_unit();
    let contract = unit
        .find_contract_by_name(contract_name)
        .next()
        .expect("can find contract");
    contract.is_payable()
}

#[test]
fn test_contract_definition_is_payable() {
    assert!(is_payable("WithReceive"));
    assert!(is_payable("WithPayableFallback"));
    assert!(!is_payable("WithPlainFallback"));
    assert!(!is_payable("WithNeither"));
    assert!(is_payable("InheritsPayable"));
}
