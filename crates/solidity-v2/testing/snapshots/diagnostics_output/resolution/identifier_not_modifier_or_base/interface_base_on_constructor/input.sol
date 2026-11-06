// SPDX-License-Identifier: MIT
pragma solidity *;

// A constructor may name an interface base, as it would a contract base.
interface I {}

contract C is I {
    constructor() I() {}
}
