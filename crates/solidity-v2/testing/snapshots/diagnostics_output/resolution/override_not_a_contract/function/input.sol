// SPDX-License-Identifier: MIT
pragma solidity *;

// An `override` list names a function rather than a contract.
contract A {
    function f() public virtual {}
}

contract B is A {
    function g() public {}

    function f() public override(g) {}
}
