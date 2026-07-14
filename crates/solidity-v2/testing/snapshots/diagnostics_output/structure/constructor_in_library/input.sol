// SPDX-License-Identifier: MIT
pragma solidity *;

// A library cannot declare a constructor.
library L {
    constructor() {}
}

// A contract may declare one, and must not be flagged.
contract C {
    constructor() {}
}
