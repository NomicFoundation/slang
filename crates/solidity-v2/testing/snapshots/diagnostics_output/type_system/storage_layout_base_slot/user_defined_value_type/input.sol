// SPDX-License-Identifier: MIT
pragma solidity *;

// The constant's type is the user-defined value type `MyType`, not an
// integer, and `MyType.wrap(5)` cannot be folded, so the base slot is
// reported as non-constant.
type MyType is uint256;
MyType constant x = MyType.wrap(5);
contract C layout at x {}
