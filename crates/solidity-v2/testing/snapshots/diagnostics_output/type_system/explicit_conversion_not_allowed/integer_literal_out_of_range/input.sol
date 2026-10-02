// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure returns (uint8, uint8) {
        return (uint8(300), uint8(255));
    }
}
