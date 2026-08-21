// SPDX-License-Identifier: MIT
pragma solidity *;

enum E { A }

function pick(bool c) pure returns (E) {
    return c ? E : E.A;
}
