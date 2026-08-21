// SPDX-License-Identifier: MIT
pragma solidity *;

// Nothing here is a duplicate: every same-named pair can be told apart.

contract Base {
    event E(uint256 a);
}

contract Derived is Base {
    event E(uint256 a, address b);
    event E(bytes32 a);
}

library L {
    event E(address a);
    event E(address a, uint256 b);
}
