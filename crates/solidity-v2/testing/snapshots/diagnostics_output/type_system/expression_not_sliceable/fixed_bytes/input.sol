// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(bytes4 b) public pure {
        // A fixed-size 'bytesN' can be indexed, but it is not an array, so it
        // cannot be sliced.
        b[0:1];
    }
}
