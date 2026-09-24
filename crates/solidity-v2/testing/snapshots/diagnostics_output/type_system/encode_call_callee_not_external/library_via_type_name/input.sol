// SPDX-License-Identifier: MIT
pragma solidity *;

library L {
    function fExternal(uint256 p, string memory t) external {}
    function fInternal(uint256 p, string memory t) internal {}
}

contract C {
    function encode() public {
        // Cannot use library functions for abi.encodeCall.
        abi.encodeCall(L.fInternal, (1, "123"));
        abi.encodeCall(L.fExternal, (1, "123"));
    }
}
