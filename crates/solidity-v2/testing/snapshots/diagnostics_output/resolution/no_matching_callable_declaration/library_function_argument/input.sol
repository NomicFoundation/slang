// SPDX-License-Identifier: MIT
pragma solidity *;

library L {
    function le() external {}
}

contract C {
    function g(function() external) internal {}
    function g(uint256) internal {}

    function test() public {
        g(L.le);
    }
}
