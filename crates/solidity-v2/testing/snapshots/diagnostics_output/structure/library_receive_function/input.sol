// SPDX-License-Identifier: MIT
pragma solidity *;

// A library cannot declare a receive function.
library L {
    receive() external payable {}
}

// A contract may declare one, and must not be flagged.
contract C {
    receive() external payable {}
}
