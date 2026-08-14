// SPDX-License-Identifier: MIT
pragma solidity *;

library L {
    function f(uint256 value) internal pure returns (uint256) {
        return value;
    }
}

// At the file level the wildcard is rejected on its own, whether or not
// specific functions are being attached.
using {L.f} for *;
