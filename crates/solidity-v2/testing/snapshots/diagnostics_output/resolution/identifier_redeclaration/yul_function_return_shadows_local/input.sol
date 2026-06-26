// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        uint256 foo;
        foo = 1;

        // Like a parameter, a Yul function return variable may not shadow a
        // *local* Solidity variable. solc accepts the declaration but rejects
        // the reference to `foo` from inside the Yul function; we reject the
        // declaration outright (a conservative approximation).
        assembly {
            function g() -> foo {
                foo := 1
            }
        }
    }
}
