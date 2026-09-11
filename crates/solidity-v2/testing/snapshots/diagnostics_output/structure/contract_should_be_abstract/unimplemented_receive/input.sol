// SPDX-License-Identifier: MIT
pragma solidity *;

// A receive without a body needs an implementation like any other function,
// so `B` must be `abstract`. `C` implements it and is fine.
abstract contract A {
    receive() external payable virtual;
}

contract B is A {}

contract C is A {
    receive() external payable override {}
}
