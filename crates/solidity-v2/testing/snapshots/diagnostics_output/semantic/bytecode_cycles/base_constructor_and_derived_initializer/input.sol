// SPDX-License-Identifier: MIT
pragma solidity *;

// Base's constructor and Mid's initializer both create D. Slang puts every
// creation unit on one queue, so Base's constructor is walked before Mid's
// initializer and Mid is attributed to Base's `new D()`. That expression was
// already reported for Base, so the second report is dropped. solc visits
// every base's initializers before any constructor body, attributes Mid to
// its own initializer, and reports all three. Both agree that Mid depends on
// D, only the expression standing for it differs.

contract Base {
    constructor() {
        new D();
    }
}

contract Mid is Base {
    D d = new D();
}

contract D {
    constructor() {
        new Mid();
    }
}
