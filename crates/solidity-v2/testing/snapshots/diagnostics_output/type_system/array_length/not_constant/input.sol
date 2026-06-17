// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    uint256 n;

    // The length is a non-constant state variable, so it is not a
    // compile-time constant.
    uint256[n] x;
}
