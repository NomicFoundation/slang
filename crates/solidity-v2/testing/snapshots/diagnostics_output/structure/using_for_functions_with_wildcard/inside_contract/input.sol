// SPDX-License-Identifier: MIT
pragma solidity *;

library L {
    function f(uint256 value) internal pure returns (uint256) {
        return value;
    }
}

contract C {
    // Attaching specific functions requires naming the target type explicitly.
    using {L.f} for *;
}
