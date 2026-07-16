// SPDX-License-Identifier: MIT
pragma solidity *;

// Valid: a non-abstract contract may specify a storage layout.
contract Concrete layout at 0x1234 {}

// Invalid: an abstract contract cannot specify a storage layout.
abstract contract Abstract layout at 0x1234 {}
