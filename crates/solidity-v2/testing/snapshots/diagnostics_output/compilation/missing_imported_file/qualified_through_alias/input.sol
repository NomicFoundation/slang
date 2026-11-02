// ---- path: main.sol

// SPDX-License-Identifier: MIT
pragma solidity *;

// The alias of a missing file is reported at the import only.
import "lib/missing.sol" as M;

contract Main {
    M.T t;
}
