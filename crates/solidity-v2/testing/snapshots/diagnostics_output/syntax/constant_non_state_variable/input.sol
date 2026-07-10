// SPDX-License-Identifier: MIT
pragma solidity *;

// solc reports DeclarationError 1788 ("The \"constant\" keyword can only be
// used for state variables or variables at file level") during analysis,
// whereas slang rejects it at the parser: the grammar does not allow
// `constant` on a function parameter.
contract C {
    function f(uint constant x) internal pure {}
}
