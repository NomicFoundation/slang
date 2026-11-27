// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function g(uint256, uint256) public {
        abi.encodeCall(this.g, (1));
    }
}
