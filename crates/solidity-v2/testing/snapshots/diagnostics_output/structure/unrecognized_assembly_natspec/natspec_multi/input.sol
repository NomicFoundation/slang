// SPDX-License-Identifier: MIT
pragma solidity *;

// solc's 'syntaxTests/inlineAssembly/natspec_multi': only the comment after the blank line
// documents the first block
function f() pure {
    /// @unrelated bogus-value

    /// @before bogus-value
    ///
    /// @solidity a   memory-safe-assembly b    c
    ///           d
    /// @after bogus-value
    assembly {}
    // solc marks this block as memory safe, but slang only recognizes a single `///` line
    /// @solidity memory-safe-assembly a a a
    ///           memory-safe-assembly
    assembly {}
}
