// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        assembly {
            let x := 1
            {
                // Unlike Solidity, shadowing in a nested block is not
                // allowed in Yul.
                let x := 2
            }
        }
    }
}
