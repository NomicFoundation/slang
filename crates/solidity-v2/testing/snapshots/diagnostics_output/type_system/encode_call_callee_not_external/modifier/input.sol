// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    modifier m(uint256 x) {
        _;
    }

    function encode() public pure returns (bytes memory) {
        // Expected first argument to be a function pointer, not "modifier (uint256)".
        return abi.encodeCall(m, (1));
    }
}
