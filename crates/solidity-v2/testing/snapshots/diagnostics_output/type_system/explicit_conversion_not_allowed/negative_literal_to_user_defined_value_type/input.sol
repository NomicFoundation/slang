// SPDX-License-Identifier: MIT
pragma solidity *;

type MyUInt is uint256;

contract C {
    function f() public pure returns (MyUInt, MyUInt) {
        // A literal does not convert to a user defined value type: `wrap` does.
        return (MyUInt(-1), MyUInt.wrap(1));
    }
}
