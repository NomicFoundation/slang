// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function internalFn(uint256 x) internal pure returns (uint256) {
        return x;
    }

    function encode() public pure returns (bytes memory) {
        // Expected regular external function type, or external view on public function.
        return abi.encodeCall(C.internalFn, (1));
    }
}
