// SPDX-License-Identifier: MIT
pragma solidity *;

contract Strings {
    string s = "café";

    // The string still ends at its closing quote, so the code after it is read as usual.
    function f() public pure {}
}
