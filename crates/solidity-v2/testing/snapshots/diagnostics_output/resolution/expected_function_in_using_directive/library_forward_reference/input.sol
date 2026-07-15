// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    using L for uint256;
}

library L {
    function f(uint256 self) internal pure returns (uint256) {
        return self;
    }
}
