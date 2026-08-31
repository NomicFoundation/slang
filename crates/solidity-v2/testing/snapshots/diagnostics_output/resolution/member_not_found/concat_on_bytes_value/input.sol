// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function missing(bytes memory data) internal pure {
        // `concat` is a member of the `bytes` type, not of a `bytes` value.
        data.concat(data);
    }

    function present(bytes memory data) internal pure returns (uint256) {
        return data.length;
    }
}
