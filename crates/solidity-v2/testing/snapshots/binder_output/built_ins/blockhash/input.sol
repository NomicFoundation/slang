// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function removed(uint256 n) internal view returns (bytes32) {
        return block.blockhash(n);      // should be rejected
    }

    function replacement(uint256 n) internal view returns (bytes32) {
        return blockhash(n);            // should be accepted
    }
}
