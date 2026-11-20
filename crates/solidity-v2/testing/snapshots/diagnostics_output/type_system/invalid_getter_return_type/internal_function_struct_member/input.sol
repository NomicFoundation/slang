// SPDX-License-Identifier: MIT
pragma solidity *;

// Unlike mappings and arrays, an internal function member is not omitted.
contract C {
    struct S {
        function() internal f;
        uint256 a;
    }

    S public s;
}
