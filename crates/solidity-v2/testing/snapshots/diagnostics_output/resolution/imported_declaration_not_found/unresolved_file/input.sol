// --- path: main.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

// The imported file itself cannot be resolved, so the import directive is
// already reported and the symbol is not flagged on top of that.
import {Missing} from "./does-not-exist.sol";

contract Test {}
