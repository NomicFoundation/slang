// SPDX-License-Identifier: MIT
pragma solidity *;

interface I {
    // A (mutable) state variable cannot be declared in an interface.
    uint x;

    // A non-`public` `constant` cannot be declared in an interface either.
    uint constant Y = 1;

    // Nor can a `public constant`.
    uint public constant Z = 2;
}

contract C {
    // State variables and constants are all fine in a contract.
    uint a;
    uint constant B = 3;
    uint public constant D = 4;
}
