// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    struct S {
        uint a;
    }

    function f() public pure {
        S memory s;
        assembly {
            let t := s
        }
    }
}
