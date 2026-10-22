// SPDX-License-Identifier: MIT
pragma solidity *;

library L {
    function l() public {}
}

contract C {
    function f() public {
        // https://github.com/argotorg/solidity/blob/v0.8.0/test/libsolidity/syntaxTests/nameAndTypeResolution/297_library_functions_do_not_have_value.sol
        // Library functions are called via `delegatecall`, so they have no `value` member.
        L.l.value;
    }
}
