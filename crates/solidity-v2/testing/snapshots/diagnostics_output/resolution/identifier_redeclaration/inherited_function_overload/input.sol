// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    function f(uint x) public {}
}

contract B is A {
    function f(bytes32 x) public {}
}
