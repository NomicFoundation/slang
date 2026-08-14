// SPDX-License-Identifier: MIT
pragma solidity *;

library L {
    function f(uint256 value) internal pure returns (uint256) {
        return value;
    }
}

contract C {
    // Attaching specific functions to a named type is allowed.
    using {L.f} for uint256;
}
