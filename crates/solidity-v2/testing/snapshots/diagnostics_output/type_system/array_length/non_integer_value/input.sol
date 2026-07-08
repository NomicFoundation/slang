// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    bool constant B = true;
    string constant S = "x";
    // Boolean and string literals/constants do not fold to an integer length.
    uint256[true] a;
    uint256["x"] b;
    uint256[B] c;
    uint256[S] d;
}
