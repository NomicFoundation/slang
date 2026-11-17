// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    struct S {
        uint256 a;
        uint256 b;
    }

    function g() public pure {
        S({a: 1, b: 2});
        S({a: 1, c: 2});
    }
}
