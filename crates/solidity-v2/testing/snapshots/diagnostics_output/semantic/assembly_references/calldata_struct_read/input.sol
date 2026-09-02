// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    struct S {
        uint a;
    }

    function f(S calldata s) external pure {
        assembly {
            let t := s
        }
    }
}
