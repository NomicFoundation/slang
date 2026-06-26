// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    uint256 foo;

    // Unlike a Yul variable, a Yul function may shadow any enclosing Solidity
    // declaration: here `foo` shadows the state variable and `bar` shadows the
    // contract function. Neither is a redeclaration.
    function bar() public pure {
        assembly {
            function foo() {}
            function bar() {}
        }
    }
}
