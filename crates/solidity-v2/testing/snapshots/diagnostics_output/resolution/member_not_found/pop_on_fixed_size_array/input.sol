// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    uint256[3] fixed_storage_array;
    uint256[] dynamic_storage_array;

    function missing() internal {
        // `pop` is only a member of dynamically-sized storage arrays.
        fixed_storage_array.pop();
    }

    function present() internal {
        dynamic_storage_array.pop();
    }
}
