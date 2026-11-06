// SPDX-License-Identifier: MIT
pragma solidity *;

// An `override` list names a struct rather than a contract.
contract A {
    function f() public virtual {}
}

contract B is A {
    struct S {
        uint256 a;
    }

    function f() public override(S) {}
}
