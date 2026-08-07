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

contract C {
    // Attaching specific functions requires naming the target type explicitly.
    using {L.f} for *;

    // Control: attaching a whole library to `*` is allowed inside a contract.
    using L for *;

    // Control: attaching specific functions to a named type is allowed.
    using {L.f} for uint256;
}
