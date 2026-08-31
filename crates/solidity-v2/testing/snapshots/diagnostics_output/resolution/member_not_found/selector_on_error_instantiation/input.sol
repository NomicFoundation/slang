// SPDX-License-Identifier: MIT
pragma solidity *;

error E(uint256 amount);

contract C {
    function missing() internal pure returns (bytes4) {
        // `selector` belongs to the error declaration, not to an instantiation of it.
        return E(1).selector;
    }

    function present() internal pure returns (bytes4) {
        return E.selector;
    }
}
