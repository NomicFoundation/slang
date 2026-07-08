// SPDX-License-Identifier: MIT
pragma solidity *;

// Valid: a single storage layout specifier is not flagged.
contract Valid layout at 0x1234 {}

// Two storage layout specifiers: the second `layout` is flagged.
contract Duplicated layout at 0x1234 layout at 0xABC {}

// Several repeated storage layout specifiers: every `layout` after the first is flagged.
contract Repeated layout at 0x1234 layout at 0xABC layout at 0x4321 layout at 0xCBA {}
