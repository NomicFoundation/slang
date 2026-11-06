// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() internal view {
        msg{gas: 1000};
    }
}
