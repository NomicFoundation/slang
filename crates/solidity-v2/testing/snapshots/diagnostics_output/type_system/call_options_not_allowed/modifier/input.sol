// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    modifier m() {
        _;
    }

    function f() internal {
        m{gas: 1000}();
    }
}
