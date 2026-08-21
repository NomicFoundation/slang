// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // Distinguishable overloads: the parameter lists differ.
    function ok(uint256 a) internal pure returns (uint256) {
        return a;
    }

    function ok(string memory a) internal pure returns (string memory) {
        return a;
    }

    function ok(uint256 a, uint256 b) internal pure returns (uint256) {
        return a + b;
    }

    // Identical parameter lists.
    function duplicated(uint256 a) internal pure returns (uint256) {
        return a;
    }

    function duplicated(uint256 b) internal pure returns (uint256) {
        return b;
    }

    // Neither the return type nor the mutability tells two overloads apart.
    function ignoresResults(uint256 a) internal pure returns (uint256) {
        return a;
    }

    function ignoresResults(uint256 a) internal view returns (bool) {
        return a == block.number;
    }

    // `memory` and `calldata` are encoded the same way at the ABI boundary.
    function ignoresLocation(uint256[] calldata a) external pure returns (uint256) {
        return a.length;
    }

    function ignoresLocation(uint256[] memory a) public pure returns (uint256) {
        return a.length;
    }

    // The same holds across visibilities: an internal overload is compared the
    // way an external one would be encoded.
    function ignoresVisibility(uint256[] calldata a) external pure returns (uint256) {
        return a.length;
    }

    function ignoresVisibility(uint256[] memory a) internal pure returns (uint256) {
        return a.length;
    }
}
