// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        assembly {
            for { let i := 0 } lt(i, 10) { i := add(i, 1) } {
                // `break`/`continue` are allowed anywhere in the loop body,
                // including inside nested blocks and `if`/`switch` statements.
                if gt(i, 5) { break }
                if lt(i, 2) { continue }

                // A nested loop keeps its own body context.
                for { let j := 0 } lt(j, 10) { j := add(j, 1) } {
                    if gt(j, 3) { break }
                    if lt(j, 1) { continue }
                }
            }
        }
    }
}
