// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    mapping(uint => uint) m;

    function f() public pure {
        assembly {
            let s := m.slot
        }
    }
}
