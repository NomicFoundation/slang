// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

contract Test {
    function copy(bytes memory data) public pure returns (bytes memory result) {
        uint256 length = data.length;

        assembly {
            result := mload(0x40)
            mstore(0x40, add(add(result, length), 32))
            mstore(result, length)
        }

        for (uint256 i = 0; i < length; i += 32) {
            bytes32 chunk;
            assembly {
                chunk := mload(add(data, add(i, 32)))
                mstore(add(result, add(i, 32)), chunk)
            }
        }
    }
}
