// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    function f() public virtual {}
}

contract C is A {
    function f() public override(A, Y) {}
}
