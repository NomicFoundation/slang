// SPDX-License-Identifier: MIT
pragma solidity *;

contract Escapes {
    // A string with both kinds of problem is flagged once, at the character.
    string s = "é \q";
}
