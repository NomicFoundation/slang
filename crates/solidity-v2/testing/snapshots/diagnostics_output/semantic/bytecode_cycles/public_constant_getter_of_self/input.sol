// SPDX-License-Identifier: MIT
pragma solidity *;

// B's own getter returns B's creation code, so B embeds itself.

contract B {
    bytes public constant CODE = type(B).creationCode;
}
