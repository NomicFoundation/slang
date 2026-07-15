// SPDX-License-Identifier: MIT
pragma solidity *;

library L {
    function f(uint256 self) internal pure returns (uint256) {
        return self;
    }
}

contract C {
    using L for uint256;
}
