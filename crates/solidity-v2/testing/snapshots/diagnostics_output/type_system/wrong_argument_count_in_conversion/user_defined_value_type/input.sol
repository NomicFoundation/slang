// SPDX-License-Identifier: MIT
pragma solidity *;

type U is uint256;

contract C {
    function g(U u) public pure {
        U(u, u);
    }
}
