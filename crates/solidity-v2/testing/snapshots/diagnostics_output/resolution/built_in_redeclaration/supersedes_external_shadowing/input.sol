// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        uint add = 1;
        assembly {
            // Should error with cannot use built-in, even though it also shadows an external variable
            let add := 1
        }
    }
}
