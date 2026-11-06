// SPDX-License-Identifier: MIT
pragma solidity *;

// A contract's state variable reached through the contract's name is not a type
// either.
contract A {
    uint256 public x;
}

contract B {
    A.x y;
}
