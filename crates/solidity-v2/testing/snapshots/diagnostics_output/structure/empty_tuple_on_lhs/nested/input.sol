// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        uint256 a;
        // Invalid: an empty tuple `()` nested as a component of an LHS tuple.
        // slang reports the empty target; solc additionally reports a type
        // error, since the right hand side cannot be assigned to this invalid
        // target.
        (a, ()) = (1, 2);
    }
}
