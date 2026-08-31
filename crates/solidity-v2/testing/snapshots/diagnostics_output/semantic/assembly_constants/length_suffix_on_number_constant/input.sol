// SPDX-License-Identifier: MIT
pragma solidity *;

// Every suffix on a constant is rejected with the same diagnostic, even
// suffixes that are never valid on constants anywhere else.
uint256 constant K = 41;

contract C {
    function f() public pure {
        assembly {
            let x := K.length
        }
    }
}
