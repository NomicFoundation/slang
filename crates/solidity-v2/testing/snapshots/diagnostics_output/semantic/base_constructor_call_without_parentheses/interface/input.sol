// SPDX-License-Identifier: MIT
pragma solidity *;

interface I {}

contract C is I {
    constructor() I {}
}
