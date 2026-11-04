// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    struct T {
        mapping(uint256 => uint256) m;
    }

    struct S {
        uint256 a;
        T[] ts;
    }

    function g() public pure {
        S(1);
    }
}
