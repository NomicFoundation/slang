// ---- path: main.sol

// SPDX-License-Identifier: MIT
pragma solidity *;

// The first import is part of the compilation, and the second one is not, so
// the diagnostic below must point at the second path, not at the first.
import "lib/present.sol";
import "lib/missing.sol";

contract Main {}

// ---- path: lib/present.sol

// SPDX-License-Identifier: MIT
pragma solidity *;

contract Present {}
