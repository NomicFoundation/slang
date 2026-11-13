// SPDX-License-Identifier: MIT
pragma solidity *;

function f() {}

contract C {
    function missing() internal {
        f{gas: 1000}();
    }
}
