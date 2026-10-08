// SPDX-License-Identifier: MIT
pragma solidity *;

interface I1 {
    function f() external;
}

interface I2 is I1 {}

contract C {
    function inherited() internal pure returns (bytes4) {
        return I2.f.selector;
    }

    function declared() internal pure returns (bytes4) {
        return I1.f.selector;
    }
}
