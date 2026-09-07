// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    fallback() external virtual {}

    receive() external payable virtual {}
}

contract B is A {
    // Both override without the `override` specifier.
    fallback() external {}

    receive() external payable {}
}
