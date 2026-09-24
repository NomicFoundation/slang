// SPDX-License-Identifier: MIT
pragma solidity *;

type T is uint256;

contract C {
    function f(uint256 x) public pure returns (T, T) {
        return (T(x), T(T.wrap(x)));
    }
}
