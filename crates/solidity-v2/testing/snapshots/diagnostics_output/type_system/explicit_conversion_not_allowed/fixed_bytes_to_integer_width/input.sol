// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint64 x) public pure returns (bytes4, bytes8) {
        return (bytes4(x), bytes8(x));
    }
}
