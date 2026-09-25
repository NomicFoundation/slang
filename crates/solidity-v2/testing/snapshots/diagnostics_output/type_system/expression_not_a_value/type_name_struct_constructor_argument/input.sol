// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    struct S { uint x; }
    function f() public pure returns (S memory) {
        return S(uint);
    }
}
