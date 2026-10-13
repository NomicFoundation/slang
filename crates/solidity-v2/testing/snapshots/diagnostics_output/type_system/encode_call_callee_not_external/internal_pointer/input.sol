// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function encode(function(uint256) internal pure returns (uint256) p)
        internal
        pure
        returns (bytes memory)
    {
        // Expected regular external function type, or external view on public function.
        return abi.encodeCall(p, (1));
    }
}
