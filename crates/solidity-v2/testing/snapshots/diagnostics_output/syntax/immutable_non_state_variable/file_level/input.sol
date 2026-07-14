// SPDX-License-Identifier: MIT
pragma solidity *;

// solc reports DeclarationError 8297 ("The \"immutable\" keyword can only be
// used for state variables") during analysis, whereas slang rejects it at the
// parser: the grammar does not allow `immutable` on a file-level variable.
uint immutable x = 7;
