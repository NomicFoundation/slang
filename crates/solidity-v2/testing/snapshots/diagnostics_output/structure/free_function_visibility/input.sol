// SPDX-License-Identifier: MIT
pragma solidity *;

// A free (file-level) function cannot specify a visibility modifier.
function f1() public {}

// Any explicit visibility on a free function is rejected.
function f2() internal {}

// A free function without a visibility modifier is allowed.
function f3() {}

contract C {
    // A visibility modifier on a contract function is allowed.
    function f4() public {}
}
