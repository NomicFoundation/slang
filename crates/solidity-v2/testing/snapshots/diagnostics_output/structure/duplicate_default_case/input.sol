// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        assembly {
            let x := 0

            // Valid: a single default case is not flagged.
            switch x
            case 0 { }
            default { }

            // Two default cases: the second `default` is flagged.
            switch x
            case 1 { }
            default { }
            default { }
        }
    }
}
