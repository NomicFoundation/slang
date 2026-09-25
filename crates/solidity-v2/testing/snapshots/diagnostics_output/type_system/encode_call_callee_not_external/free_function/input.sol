// SPDX-License-Identifier: MIT
pragma solidity *;

function freeFn(uint256 x) pure returns (uint256) {
    return x;
}

contract C {
    function encode() public pure returns (bytes memory) {
        // Expected regular external function type, or external view on public function.
        return abi.encodeCall(freeFn, (1));
    }
}
