// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    uint256[] dynamic_storage_array;

    function missing(uint256 value) internal pure {
        value.pop();
    }

    function present() internal {
        dynamic_storage_array.pop();
    }
}
