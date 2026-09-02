// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    uint256 internal constant X = 1;

    function f() internal pure virtual {}
}

contract B is A {
    function missing() internal pure returns (uint256) {
        // `super` reaches functions, not constants.
        return super.X;
    }

    function present() internal pure {
        super.f();
    }
}
