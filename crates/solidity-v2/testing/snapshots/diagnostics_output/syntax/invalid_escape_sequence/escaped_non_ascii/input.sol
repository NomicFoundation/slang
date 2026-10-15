// SPDX-License-Identifier: MIT
pragma solidity *;

contract Escapes {
    // A backslash before a character outside of printable ASCII is an invalid escape.
    string s = "\é";
}
