// SPDX-License-Identifier: MIT
pragma solidity *;

// Recovered from solc:
// test/libsolidity/syntaxTests/types/contractTypeType/members/call_function_via_contract_name.sol
// Calling external/public functions of an unrelated contract via its type name.

contract A {
    function f() external {}
    function g() external pure {}
    function h() public pure {}
}

contract B {
    function i() external {
        // TypeError 3419: Cannot call function via contract type name.
        A.f();
        A.g();
        A.h(); // might be allowed in the future
    }
}
