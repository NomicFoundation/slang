// SPDX-License-Identifier: MIT
pragma solidity *;

// Recovered from solc:
// test/libsolidity/syntaxTests/types/contractTypeType/members/base_contract_invalid.sol
// `B.f()` calls an external base function via the type name (error 3419).
// `B.g.selector` additionally fails in solc with error 9582 (no `selector`
// member on an internal function) — a separate diagnostic not modelled here;
// the file still fails on the 3419, matching solc's overall failure status.

contract B {
    function f() external {}
    function g() internal {}
}

contract C is B {
    function i() public {
        // TypeError 3419: Cannot call function via contract type name.
        B.f();
        B.g.selector;
    }
}
