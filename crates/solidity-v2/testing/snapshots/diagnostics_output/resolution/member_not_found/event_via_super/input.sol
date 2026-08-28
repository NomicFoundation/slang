// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    event E(uint256 value);

    function f() internal pure virtual {}
}

contract B is A {
    function missing() internal pure returns (bytes32) {
        // `super` reaches functions, not events.
        return super.E.selector;
    }

    function present() internal pure {
        super.f();
    }
}
