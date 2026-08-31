// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    uint256[] dynamic_storage_array;

    function missing() internal pure {
        // `push` is only a member of storage arrays.
        uint256[] memory memory_array = new uint256[](1);
        memory_array.push(1);
    }

    function present() internal {
        dynamic_storage_array.push(1);
    }
}
