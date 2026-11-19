// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        // A type name can only be indexed to name an array type, not sliced.
        uint256[1:];
    }
}
