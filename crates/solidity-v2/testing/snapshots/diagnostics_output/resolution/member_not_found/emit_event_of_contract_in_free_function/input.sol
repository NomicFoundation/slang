// SPDX-License-Identifier: MIT
pragma solidity *;

contract D {
    event E();
}

function f() {
    emit D.E();
}
