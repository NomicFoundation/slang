// SPDX-License-Identifier: MIT
pragma solidity *;

abstract contract Other {
    function f() public virtual;
}

contract Concrete {}

contract C {
    function missing() internal pure returns (bytes4) {
        return type(Concrete).interfaceId;
    }

    function present() internal pure returns (bytes4) {
        return type(Other).interfaceId;
    }
}
