// SPDX-License-Identifier: MIT
pragma solidity *;

// solc's 'syntaxTests/inlineAssembly/invalid_natspec'
contract C {
    function f() public pure {
        /// @test test
        assembly {}
        /// @solidity test
        assembly {}
        /// @param
        assembly {}
    }
}
