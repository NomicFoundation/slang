// SPDX-License-Identifier: MIT
pragma solidity *;

type U is uint256;

contract C {
    function g() public pure {
        U.wrap();
    }
}
