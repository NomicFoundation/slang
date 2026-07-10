// SPDX-License-Identifier: MIT
pragma solidity *;

// A file-level `constant` without an initializer. Unlike contract-level
// constants (see `uninitialized_constant`), slang's grammar requires the
// `= value` on a file-level `ConstantDefinition`, so slang reports a parse
// error while solc parses the declaration and reports a semantic error. Both
// still fail to compile; only the message diverges.

uint256 constant A;
