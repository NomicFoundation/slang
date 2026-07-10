// SPDX-License-Identifier: MIT
pragma solidity *;

// solc reports DeclarationError 8342 ("Only constant variables are allowed at
// file level") during analysis, whereas slang rejects it at the parser: the
// grammar only allows constant variables at file level.
uint x = 7;
