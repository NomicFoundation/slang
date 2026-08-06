// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        assembly {
            if 1 { break }
            if 1 { continue }
        }
    }
}
