// SPDX-License-Identifier: MIT
pragma solidity *;

// Interface members are implicitly virtual, so the receive still needs an
// implementation and `B` must be `abstract`. `C` implements it and is fine.
interface I {
    receive() external payable;
}

contract B is I {}

contract C is I {
    receive() external payable {}
}
