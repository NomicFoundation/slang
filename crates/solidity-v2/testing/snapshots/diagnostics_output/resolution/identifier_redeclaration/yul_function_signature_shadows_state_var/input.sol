// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    uint256 foo;
    uint256 bar;

    // A Yul function's parameters and return variables may shadow non-local
    // Solidity declarations: here the parameter `foo` and the return variable
    // `bar` shadow state variables. Neither is a redeclaration.
    function f() public pure {
        assembly {
            function g(foo) -> bar {
                bar := foo
            }
        }
    }
}
