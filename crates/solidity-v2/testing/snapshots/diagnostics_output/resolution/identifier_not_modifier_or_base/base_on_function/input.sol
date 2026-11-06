// SPDX-License-Identifier: MIT
pragma solidity *;

// Only a constructor can invoke a base's constructor, so a base named by a
// function's modifiers is rejected.
contract A {}

contract C is A {
    function f() public A {}
}
