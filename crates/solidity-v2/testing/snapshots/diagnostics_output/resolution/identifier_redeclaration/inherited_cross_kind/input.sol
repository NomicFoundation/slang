// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    function f() public {}
}

contract B is A {
    uint f;
}
