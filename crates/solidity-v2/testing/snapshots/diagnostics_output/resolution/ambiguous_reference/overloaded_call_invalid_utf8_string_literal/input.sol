// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function g(string memory) internal pure {}

    function g(bytes memory) internal pure {}

    function ambiguous() internal pure {
        // Valid UTF-8 converts to both `string` and `bytes`, so both overloads
        // accept the call.
        g(hex"41");
    }

    function unambiguous() internal pure {
        // Invalid UTF-8 only converts to `bytes`, so only one overload accepts it.
        g(hex"ff");
    }
}
