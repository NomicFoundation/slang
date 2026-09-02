// SPDX-License-Identifier: MIT
pragma solidity *;

enum E { A }

// A user-defined type name in one branch.
function pick(bool c) pure returns (E) {
    return c ? E : E.A;
}

// The order of the branches does not matter.
function pickMixed(bool c) pure returns (uint8) {
    return c ? uint8(1) : E;
}

// Both branches are reported, even when they name the same type. Last, since
// solc stops checking the file once both branches of a conditional are invalid.
function pickBoth(bool c) pure returns (E) {
    return c ? E : E;
}
