// SPDX-License-Identifier: MIT
pragma solidity *;

library L {
    // A non-constant state variable is not allowed in a library.
    uint x;

    // A `constant` state variable is allowed in a library.
    uint constant Y = 1;

    // A `public constant` is allowed in a library too.
    uint public constant Z = 2;
}

contract C {
    // Non-constant state variables are fine in a contract.
    uint a;
}
