// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure returns (bytes2, bytes2) {
        // An odd number of hex digits spans no whole number of bytes.
        return (bytes2(0x123), bytes2(0x0123));
    }
}
