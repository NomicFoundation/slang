// SPDX-License-Identifier: MIT
pragma solidity *;

library L {
    function duplicated(uint256 a) internal pure returns (uint256) {
        return a;
    }

    function duplicated(uint256 a) internal pure returns (uint256) {
        return a + 1;
    }

    function ok(bytes32 a) internal pure returns (bytes32) {
        return a;
    }
}

interface I {
    function duplicated(address a) external returns (bool);

    function duplicated(address b) external returns (bool);

    function ok(address a, uint256 b) external returns (bool);
}
