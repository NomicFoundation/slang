// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        uint256 foo;
        foo = 1;

        // A Yul function parameter may not shadow a *local* Solidity variable.
        // solc accepts the declaration but rejects the reference to `foo` from
        // inside the Yul function ("Cannot access local Solidity variables...");
        // we reject the declaration outright (a conservative approximation).
        assembly {
            function g(foo) {
                pop(foo)
            }
        }
    }
}
