// SPDX-License-Identifier: MIT
pragma solidity *;

// A nested struct is returned whole, so its mapping cannot be omitted.
contract C {
    struct T {
        mapping(uint256 => uint256) m;
    }

    struct S {
        T t;
        uint256 a;
    }

    S public s;
}
