// SPDX-License-Identifier: MIT
pragma solidity *;

contract Escapes {
    // Unicode string literals can hold any character, but not any escape sequence.
    string s = unicode"café \✅";
}
