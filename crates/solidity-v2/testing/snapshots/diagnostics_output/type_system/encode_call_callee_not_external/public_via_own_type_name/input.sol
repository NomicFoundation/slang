// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function publicFn(uint256 x) public pure returns (uint256) {
        return x;
    }

    function encode() public pure returns (bytes memory) {
        // Expected regular external function type, or external view on public function.
        return abi.encodeCall(C.publicFn, (1));
    }
}
