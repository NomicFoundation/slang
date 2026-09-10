// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    uint256 public foo;
}

contract B is A {
    // A public state variable cannot be overridden by another one either.
    uint256 public override foo;
}
