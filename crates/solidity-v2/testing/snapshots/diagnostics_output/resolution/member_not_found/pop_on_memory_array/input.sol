// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    uint256[] dynamic_storage_array;

    function missing() internal pure {
        // `pop` is only a member of storage arrays.
        uint256[] memory memory_array = new uint256[](1);
        memory_array.pop();
    }

    function present() internal {
        dynamic_storage_array.pop();
    }
}
