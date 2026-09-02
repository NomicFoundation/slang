// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // A hex literal of matching width converts to a byte array on its own, but
    // the conditional measures the mobile type of each branch, and `uint8`
    // does not.
    function f() public pure returns (bytes1) {
        return true ? bytes1(0x01) : 0x01;
    }
}
