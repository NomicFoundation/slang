// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(address payable a) public pure returns (bytes20, bytes20) {
        // Only a non-payable address converts to `bytes20`.
        return (bytes20(a), bytes20(address(a)));
    }
}
