// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() internal {}

    function g() public {
        // Neither set of options is allowed, which is reported for each.
        f{gas: 1}{gas: 2}();
    }
}
