// SPDX-License-Identifier: MIT
pragma solidity *;

// One declaration can be both a redeclaration and a duplicate: the two checks
// answer different questions, so each reports on its own.

contract Base {
    event E(uint256 a);
}

// A struct can't share a name with an inherited event, so this redeclares it.
// The name now holds members of differing kinds, and no longer admits any.
contract Middle is Base {
    struct E {
        uint256 x;
    }
}

// `E` therefore redeclares the inherited name, *and* shares an ABI slot with
// `Base.E`, so it is reported twice.
contract Derived is Middle {
    event E(uint256 a);
}
