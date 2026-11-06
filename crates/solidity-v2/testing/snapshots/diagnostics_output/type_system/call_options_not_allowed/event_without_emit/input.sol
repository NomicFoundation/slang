// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    event E();

    function missing() internal {
        E{gas: 1000}();
    }
}
