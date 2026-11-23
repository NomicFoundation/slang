// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    struct S {
        uint256 a;
        bool b;
    }

    function g() public pure {
        S({b: 1, a: 2});
    }
}
