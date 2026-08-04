// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        assembly {
            let x := 0

            // Valid: the default case is the last case.
            switch x
            case 0 { }
            default { }

            // A `case` following the `default` case: the trailing `case` is flagged.
            switch x
            default { }
            case 1 { }
        }
    }
}
