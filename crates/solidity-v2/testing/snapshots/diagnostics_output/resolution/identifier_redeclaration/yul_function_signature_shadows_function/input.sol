// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // A Yul function's return variable (or parameter) may shadow a Solidity
    // function name; functions and Yul locals are in separate namespaces.
    // Mirrors solx-solidity:
    // test/libsolidity/semanticTests/inlineAssembly/external_identifier_access_shadowing.sol
    function f() public pure returns (uint256 x) {
        assembly {
            function g() -> f {
                f := 2
            }
            x := g()
        }
    }
}
