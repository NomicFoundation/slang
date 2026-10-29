// SPDX-License-Identifier: MIT
pragma solidity *;

// The local `N` in an `unchecked` block shadows the constant, so the length is not a constant.
contract C {
    uint256 constant N = 2;

    function f() public pure {
        unchecked {
            uint256 N = 1;
            uint256[N] memory arr;
        }
    }
}
