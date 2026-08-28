// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    function internal_function() internal pure {}

    function external_function() external pure {}
}

contract C {
    function missing(A a) internal pure {
        // Only externally visible members are reachable through an instance.
        a.internal_function();
    }

    function present(A a) internal {
        a.external_function();
    }
}
