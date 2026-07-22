// SPDX-License-Identifier: MIT
pragma solidity *;

function f(uint256 self) pure returns (uint256) {
    return self;
}

library L {
    function g(uint256 self) internal pure returns (uint256) {
        return self;
    }
}

contract C {
    using {f, L.g} for uint256;
}
