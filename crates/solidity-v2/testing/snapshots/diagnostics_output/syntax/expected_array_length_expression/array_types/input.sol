// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        uint[][1:] memory a;
        uint[][1:2] memory b;
        uint[1:] memory c;
        uint[1:2] memory d;
        uint[1:][] memory e;
        uint[1:2][] memory f;
    }
}
