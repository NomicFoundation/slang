// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(bytes[] calldata a) public pure {
        // The 'bytes' elements are dynamically encoded, so the array cannot be
        // sliced.
        a[1:];
    }
}
