// SPDX-License-Identifier: MIT
pragma solidity *;

// A variable initialized from a slice is still declared as a calldata array,
// so it takes the same suffixes as one.
contract C {
    function f(bytes calldata b) external pure {
        bytes calldata s = b[1:];
        assembly {
            let o := s.offset
            let l := s.length
        }
    }
}
