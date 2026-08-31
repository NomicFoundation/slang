// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    event E(uint256 amount);

    function missing() internal pure returns (bytes32) {
        // `selector` belongs to the event declaration, not to an invocation of it.
        return E(1).selector;
    }

    function present() internal {
        emit E(1);
    }
}
