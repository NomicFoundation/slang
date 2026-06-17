// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // The length references a constant declared later in the file. The
    // constant evaluator only folds constants declared before the use site,
    // so the length is not a compile-time constant.
    uint256[LEN] x;
}

uint256 constant LEN = 5;
