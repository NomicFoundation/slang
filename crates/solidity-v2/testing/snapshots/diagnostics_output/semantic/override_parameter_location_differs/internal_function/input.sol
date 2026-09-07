// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    function f(uint256[1] memory a) internal pure virtual returns (uint256) {
        return a[0];
    }
}

// The two `f` have the same signature, but a function overriding a
// non-external one has to keep the data locations of its parameters. This is
// enforced from 0.8.14 on.
contract B is A {
    function f(uint256[1] calldata a) internal pure override returns (uint256) {
        return a[0];
    }
}
