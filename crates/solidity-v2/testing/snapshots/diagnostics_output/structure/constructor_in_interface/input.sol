// SPDX-License-Identifier: MIT
pragma solidity *;

// An interface cannot declare a constructor.
interface I {
    constructor() {}
}

// A contract may declare one, and must not be flagged.
contract C {
    constructor() {}
}
