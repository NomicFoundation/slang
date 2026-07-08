// SPDX-License-Identifier: MIT
pragma solidity *;

interface I {
    function foo() external;
}

// `C` does not implement the function declared in the interface.
contract C is I {}
