// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        assembly {
            for { let i := 0 break } lt(i, 10) { i := add(i, 1) } { }
            for { let j := 0 continue } lt(j, 10) { j := add(j, 1) } { }
        }
    }
}
