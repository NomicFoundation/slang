// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {}

contract C is A {
    function f() public A.m {}
}
