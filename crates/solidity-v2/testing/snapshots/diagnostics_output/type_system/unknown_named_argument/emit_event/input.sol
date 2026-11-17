// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    event E(uint256 a);

    function g() public {
        emit E({a: 1});
        emit E({b: 1});
    }
}
