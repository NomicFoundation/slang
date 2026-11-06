// SPDX-License-Identifier: MIT
pragma solidity *;

contract Escapes {
    // A backslash before a character that starts no escape sequence.
    string s = "\q";
}
