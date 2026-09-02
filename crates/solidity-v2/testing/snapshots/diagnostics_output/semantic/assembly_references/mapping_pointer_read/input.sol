// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    mapping(uint => uint) m;

    function f() public {
        mapping(uint => uint) storage p = m;
        assembly {
            pop(p)
        }
    }
}
