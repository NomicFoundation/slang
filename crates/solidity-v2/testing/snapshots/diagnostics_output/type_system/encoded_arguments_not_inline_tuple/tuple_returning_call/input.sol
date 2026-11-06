// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function g(uint256, bool) public {
        abi.encodeCall(this.g, t());
    }

    function t() internal returns (uint256, bool) {}
}
