// SPDX-License-Identifier: MIT
pragma solidity *;

// Derived inherits the constant. Its getter serves the selector in both
// contracts, so Derived's deployed code embeds Derived's own creation code.

contract Base {
    bytes public constant CODE = type(Derived).creationCode;
}

contract Derived is Base {}
