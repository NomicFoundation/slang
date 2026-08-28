// SPDX-License-Identifier: MIT
pragma solidity *;

abstract contract Other {
    function f() public virtual;
}

contract Concrete {}

contract C {
    function missing() internal pure returns (bytes memory) {
        // An abstract contract has no bytecode of its own.
        return type(Other).runtimeCode;
    }

    function present() internal pure returns (bytes memory) {
        return type(Concrete).runtimeCode;
    }
}
