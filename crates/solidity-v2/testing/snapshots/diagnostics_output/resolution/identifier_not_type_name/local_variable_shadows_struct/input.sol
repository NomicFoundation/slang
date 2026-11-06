// SPDX-License-Identifier: MIT
pragma solidity *;

// The local variable `S` shadows the struct, so the type name finds the
// variable rather than looking further out.
struct S {
    uint256 a;
}

contract C {
    function f() public pure {
        uint256 S;
        S memory s;
    }
}
