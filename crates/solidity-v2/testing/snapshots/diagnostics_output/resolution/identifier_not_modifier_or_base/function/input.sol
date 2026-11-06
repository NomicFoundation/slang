// SPDX-License-Identifier: MIT
pragma solidity *;

// A modifier invocation names a function.
contract C {
    function g() public {}

    function f() public g {}
}
