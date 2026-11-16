// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint256[2] calldata a) public pure {
        // A fixed-size array cannot be sliced, even in calldata.
        a[1:];
    }
}
