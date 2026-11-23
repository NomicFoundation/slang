// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    event E(uint256 a);

    function g() public {
        E(true);
    }
}
