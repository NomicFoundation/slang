// SPDX-License-Identifier: MIT
pragma solidity *;

// Recovered from solc:
// test/libsolidity/syntaxTests/freeFunctions/free_call_via_contract_type.sol
// Calling a function through a contract type name from a free function.

contract C {
    function f() public pure {}
}

function fun() {
    // TypeError 3419: Cannot call function via contract type name.
    C.f();
}
