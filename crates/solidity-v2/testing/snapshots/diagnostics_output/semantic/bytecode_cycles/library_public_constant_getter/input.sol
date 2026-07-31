// SPDX-License-Identifier: MIT
pragma solidity *;

// A library's public constant has a getter too, so L embeds itself.

library L {
    bytes public constant CODE = type(L).creationCode;
}
