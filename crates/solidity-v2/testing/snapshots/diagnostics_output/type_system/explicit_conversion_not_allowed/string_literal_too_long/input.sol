// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure returns (bytes2, bytes32) {
        return (bytes2("abc"), bytes32("abc"));
    }
}
