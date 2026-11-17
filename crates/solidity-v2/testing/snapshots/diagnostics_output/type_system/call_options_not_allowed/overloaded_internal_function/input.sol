// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function h(uint256) internal {}

    function h(bytes32) internal {}

    function e(uint256) external {}

    function e(bytes32) external {}

    function missing() internal {
        h{gas: 1000}(1);
    }

    function present() internal {
        this.e{gas: 1000}(1);
    }
}
