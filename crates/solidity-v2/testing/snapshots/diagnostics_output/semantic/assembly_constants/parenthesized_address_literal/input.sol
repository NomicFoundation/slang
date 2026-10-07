// SPDX-License-Identifier: MIT
pragma solidity *;

// Without the parentheses this is a literal; with them it is just a value of type `address`.
address constant K = (0x1111111111111111111111111111111111111111);

contract C {
    function f() public pure {
        assembly {
            let x := K
        }
    }
}
