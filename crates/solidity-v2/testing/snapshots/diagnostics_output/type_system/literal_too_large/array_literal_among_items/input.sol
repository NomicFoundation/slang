// SPDX-License-Identifier: MIT
pragma solidity *;

// The declared type of the variable does not change how the array literal is
// typed: its items still need a mobile type of their own.

function f() pure {
    uint256[2] memory a = [1, 2**256];
}
