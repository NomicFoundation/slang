// SPDX-License-Identifier: MIT
pragma solidity *;

function f(uint256 self, uint256 x) pure returns (uint256) {
    return self + x;
}

contract C {
    using {f} for uint256;

    function encode() public pure returns (bytes memory) {
        uint256 x = 1;
        // Expected regular external function type, or external view on public function.
        return abi.encodeCall(x.f, (1));
    }
}
