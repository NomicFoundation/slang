// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    struct S {
        uint256 a;
    }
}

contract C {
    function g() public pure {
        A.S(true);
    }
}
