// SPDX-License-Identifier: MIT
pragma solidity *;

// The struct only names itself in a function type, which holds no value of it.
contract C {
    struct T {
        function(T memory) external f;
        uint256 a;
    }

    struct S {
        T t;
        uint256 b;
    }

    S public s;
}
