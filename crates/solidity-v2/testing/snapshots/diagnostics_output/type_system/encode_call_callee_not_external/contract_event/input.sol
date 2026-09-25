// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    event E(uint256 a);

    function encode() public pure returns (bytes memory) {
        // Cannot use events for abi.encodeCall.
        return abi.encodeCall(E, (1));
    }
}
