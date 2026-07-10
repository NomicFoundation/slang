// SPDX-License-Identifier: MIT
pragma solidity *;

// A `private` base constant is not inherited, so reusing the name in the
// derived contract is legal (the array is `uint256[2]`).
contract Base {
    uint256 private constant N = 1;
}
contract Derived is Base {
    uint256 constant N = 2;
    uint256[N] a;
}
