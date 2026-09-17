// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(string calldata s) external pure {
        assembly {
            let t := s.slot
        }
    }
}
