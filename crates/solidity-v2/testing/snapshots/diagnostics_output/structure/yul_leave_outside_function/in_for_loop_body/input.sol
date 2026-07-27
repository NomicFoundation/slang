// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint256 n) public pure {
        assembly {
            // Unlike `break`/`continue`, a for-loop body does not make `leave`
            // valid; it must be inside a function.
            for { } n { } { leave }
        }
    }
}
