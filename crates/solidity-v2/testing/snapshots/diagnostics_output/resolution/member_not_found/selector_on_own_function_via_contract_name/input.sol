// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function g() public {}

    function missing() internal pure returns (bytes4) {
        // Only a function of another contract has a selector this way.
        return C.g.selector;
    }
}

contract D is C {
    function present() internal pure returns (bytes4) {
        // From a derived contract, `g` belongs to another contract.
        return C.g.selector;
    }
}
