// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    struct S {
        uint256 a;
        mapping(uint256 => uint256) m;
    }

    function g() public pure {
        S({a: 1});
    }
}
