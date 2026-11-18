// SPDX-License-Identifier: MIT
pragma solidity *;

// `2**256` is a valid number literal, but no integer type can hold it, so it
// cannot be an item of an array literal.

function f() pure {
    [2**256];
}
