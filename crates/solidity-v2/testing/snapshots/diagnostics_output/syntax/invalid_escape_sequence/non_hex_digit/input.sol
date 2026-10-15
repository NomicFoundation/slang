// SPDX-License-Identifier: MIT
pragma solidity *;

contract Escapes {
    // A hex escape cut short by a character that is not a hex digit.
    string s = "\x4g";
}
