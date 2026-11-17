// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    error E();

    function g() public pure {
        require(true, E());
    }
}
