// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        uint[] memory y = new uint[](1);
        assembly {
            let t := y.slot
        }
    }
}
