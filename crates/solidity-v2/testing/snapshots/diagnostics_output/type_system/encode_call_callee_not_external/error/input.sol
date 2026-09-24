// SPDX-License-Identifier: MIT
pragma solidity *;

error E(uint256 a);

contract C {
    function encode() public pure returns (bytes memory) {
        // Cannot use errors for abi.encodeCall.
        return abi.encodeCall(E, (1));
    }
}
