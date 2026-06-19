// SPDX-License-Identifier: MIT
pragma solidity *;

// Recovered from solc:
// test/libsolidity/syntaxTests/nameAndTypeResolution/145_external_base_visibility.sol
// Calling an external base function through the base contract type name.

contract base {
    function ext() external {}
    function intl() external {}
    function pub() public {}
}

contract derived is base {
    function g() public {
        // internal and public are ok
        base.intl();
        base.pub();
        // Cannot call external function via contract type name.
        base.f();
    }
}
