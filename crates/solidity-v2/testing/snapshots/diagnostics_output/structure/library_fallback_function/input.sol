// SPDX-License-Identifier: MIT
pragma solidity *;

// A library cannot declare a fallback function.
library L {
    fallback() external {}
}

// A contract may declare one, and must not be flagged.
contract C {
    fallback() external {}
}
