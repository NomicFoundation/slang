// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    uint256 constant X = 10;
}

// The base slot references a constant through member access, which the
// constant evaluator cannot fold.
contract C is A layout at A.X {}
