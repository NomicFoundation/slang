// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {}

// solc's `invalid_multiline_comment_close` scanner test: `/ ` does not close the comment
/** / x
