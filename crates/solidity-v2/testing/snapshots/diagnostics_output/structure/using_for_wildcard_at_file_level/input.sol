// SPDX-License-Identifier: MIT
pragma solidity *;

library L {}

// A file-level `using` directive must name the target type explicitly.
using L for *;

// Control: naming the type explicitly at the file level is allowed.
using L for uint256;
