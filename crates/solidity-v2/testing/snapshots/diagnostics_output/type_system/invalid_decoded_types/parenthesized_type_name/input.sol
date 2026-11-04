// SPDX-License-Identifier: MIT
pragma solidity *;

// Extra parentheses around a single type name only group it, at any depth.
function test(bytes memory data) pure {
    abi.decode(data, ((uint256)));
    abi.decode(data, (((uint256))));
}
