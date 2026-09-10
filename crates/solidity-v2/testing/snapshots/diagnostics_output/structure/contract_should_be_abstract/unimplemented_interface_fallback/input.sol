// SPDX-License-Identifier: MIT
pragma solidity *;

// Interface members are implicitly virtual, so the fallback still needs an
// implementation and `B` must be `abstract`. `C` implements it and is fine.
interface I {
    fallback() external;
}

contract B is I {}

contract C is I {
    fallback() external {}
}
