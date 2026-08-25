// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    event E(uint256 u, string s);
    event E(string s, uint256 u);

    function unambiguous() internal {
        // Positional arguments pick the overload by parameter order.
        emit E(2, "abc");
    }

    function ambiguous() internal {
        // Both overloads declare the same parameter names, so naming the
        // arguments doesn't tell them apart.
        emit E({u: 2, s: "abc"});
    }
}
