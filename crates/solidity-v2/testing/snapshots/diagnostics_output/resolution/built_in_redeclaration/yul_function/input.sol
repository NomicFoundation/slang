// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        assembly {
            function add() -> result {
                result := 1
            }
        }
    }
}
