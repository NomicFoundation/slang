// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    struct T {
        uint256 x;
    }

    struct U {
        uint256 x;
    }

    struct S {
        T t;
    }

    function g() public pure {
        S(U(1));
    }
}
