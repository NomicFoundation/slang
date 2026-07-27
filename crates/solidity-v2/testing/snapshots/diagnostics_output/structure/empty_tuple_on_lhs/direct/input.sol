// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        // slang flags the empty tuple `()` as an assignment target. solc
        // rejects the bare `()` earlier, at parse time; both report exactly one
        // diagnostic.
        () = ();
    }
}
