// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // The literal `0` converts to a byte array on its own, but the conditional
    // measures the mobile type of each branch, and `uint8` does not.
    function f() public pure returns (bytes32) {
        return true ? bytes32(0) : 0;
    }
}
