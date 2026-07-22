// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    using {f} for uint256;

    function f(uint256 self) internal pure returns (uint256) {
        return self;
    }
}
