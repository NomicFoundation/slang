// SPDX-License-Identifier: MIT
pragma solidity *;

// Recovered from solc:
// test/libsolidity/syntaxTests/nameAndTypeResolution/145_external_base_visibility.sol
// Calling an external base function through the base contract type name.

contract base {
    function f() external {}
}

contract derived is base {
    function g() public {
        // TypeError 3419: Cannot call function via contract type name.
        base.f();
    }
}
