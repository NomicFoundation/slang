// SPDX-License-Identifier: MIT
pragma solidity *;

library L {
    function fExternal(uint256 p, string memory t) external {}
}

contract C {
    using L for uint256;

    function encode() public returns (bytes memory) {
        uint256 x = 1;
        // Cannot use library functions for abi.encodeCall.
        return abi.encodeCall(x.fExternal, (1, "123"));
    }
}
