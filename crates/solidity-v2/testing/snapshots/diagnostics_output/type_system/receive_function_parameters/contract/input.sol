// SPDX-License-Identifier: MIT
pragma solidity *;

// A receive function cannot take parameters.
contract A {
    receive(uint256) external payable {}
}

// A parameterless receive is valid and must not be flagged.
contract C {
    receive() external payable {}
}
