// SPDX-License-Identifier: MIT
pragma solidity *;

library L {
    event E(uint256 a);
}

contract C {
    function encode() public pure returns (bytes memory) {
        // Cannot use events for abi.encodeCall.
        return abi.encodeCall(L.E, (1));
    }
}
