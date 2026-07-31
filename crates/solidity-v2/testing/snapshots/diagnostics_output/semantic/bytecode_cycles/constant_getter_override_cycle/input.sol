// SPDX-License-Identifier: MIT
pragma solidity *;

// The constant overrides x. Its getter serves the selector, so B's deployed
// code embeds B's own creation code.

abstract contract Base {
    function x() external pure virtual returns (bytes memory);
}

contract B is Base {
    bytes public constant override x = type(B).creationCode;
}
