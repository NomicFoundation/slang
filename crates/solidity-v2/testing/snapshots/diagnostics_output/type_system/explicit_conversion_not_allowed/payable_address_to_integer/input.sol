// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(address payable a) public pure returns (uint160, uint160) {
        // Only a non-payable address converts to `uint160`.
        return (uint160(a), uint160(address(a)));
    }
}
