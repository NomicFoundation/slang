// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    struct S {
        uint256 a;
        uint256 b;
    }

    function g() public pure {
        S(1);
    }
}
