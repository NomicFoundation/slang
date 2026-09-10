// SPDX-License-Identifier: MIT
pragma solidity *;

// A fallback without a body needs an implementation like any other function,
// so `B` must be `abstract`. `C` implements it and is fine.
abstract contract A {
    fallback() external virtual;
}

contract B is A {}

contract C is A {
    fallback() external override {}
}
