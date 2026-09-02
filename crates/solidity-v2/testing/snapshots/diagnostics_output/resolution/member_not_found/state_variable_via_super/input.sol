// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    uint256 internal x;

    function f() internal pure virtual {}
}

contract B is A {
    function missing() internal view returns (uint256) {
        // `super` reaches functions, not state variables.
        return super.x;
    }

    function present() internal pure {
        super.f();
    }
}
