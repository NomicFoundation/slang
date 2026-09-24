// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure returns (bytes32, bytes32, bytes2) {
        // Only zero, or a hex literal of the exact width, converts.
        return (bytes32(1), bytes32(0), bytes2(0x1234));
    }
}
