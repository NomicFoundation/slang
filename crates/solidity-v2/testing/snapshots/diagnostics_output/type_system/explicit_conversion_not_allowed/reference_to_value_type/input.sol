// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint256[] memory a) public pure returns (uint256, uint256) {
        return (uint256(a), uint256(a.length));
    }
}
