// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(bytes calldata b) external pure {
        bytes calldata s = b[1:];
        assembly {
            let t := s.slot
        }
    }
}
