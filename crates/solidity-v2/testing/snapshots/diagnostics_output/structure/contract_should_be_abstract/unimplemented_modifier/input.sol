// SPDX-License-Identifier: MIT
pragma solidity *;

// The modifier is declared without a body (and marked `virtual`, so the only
// remaining error is that the contract must be `abstract`).
contract A {
    modifier m virtual;
}
