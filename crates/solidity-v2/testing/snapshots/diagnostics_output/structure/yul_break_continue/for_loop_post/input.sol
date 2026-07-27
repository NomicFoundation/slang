// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        assembly {
            for { let i := 0 } lt(i, 10) { i := add(i, 1) break } { }
            for { let j := 0 } lt(j, 10) { j := add(j, 1) continue } { }
        }
    }
}
