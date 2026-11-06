// SPDX-License-Identifier: MIT
pragma solidity *;

// A modifier invocation names a library, which has no constructor to invoke.
library L {}

contract C {
    function f() public L {}
}
