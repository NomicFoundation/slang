// SPDX-License-Identifier: MIT
pragma solidity *;

// A constant without a getter is not an entry point. An unreferenced value is
// never compiled into any bytecode, so B does not embed its own creation code.

contract B {
    bytes constant c = type(B).creationCode;
}
