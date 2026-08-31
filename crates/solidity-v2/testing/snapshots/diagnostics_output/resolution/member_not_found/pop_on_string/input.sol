// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    string storage_string;
    bytes storage_bytes;

    function missing() internal {
        // `string` has no members, unlike `bytes`.
        storage_string.pop();
    }

    function present() internal {
        storage_bytes.pop();
    }
}
