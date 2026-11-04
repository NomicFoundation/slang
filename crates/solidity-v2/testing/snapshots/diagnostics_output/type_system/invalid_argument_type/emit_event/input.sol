// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    event E(uint256 a);
    event F(string a);

    function g(string calldata s) public {
        emit F(s);
        emit E(true);
    }
}
