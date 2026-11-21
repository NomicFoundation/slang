// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    struct S {
        mapping(uint256 => uint256) m;
    }

    S public s;
}

// The struct has no member a getter can return, so there is no getter to call.
contract D {
    function test(C c) public {
        c.s();
    }
}
