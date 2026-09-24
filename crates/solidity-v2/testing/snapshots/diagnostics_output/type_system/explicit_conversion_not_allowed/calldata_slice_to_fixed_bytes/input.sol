// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(bytes calldata b) external pure returns (bytes4) {
        return bytes4(b[:4]);
    }
}
