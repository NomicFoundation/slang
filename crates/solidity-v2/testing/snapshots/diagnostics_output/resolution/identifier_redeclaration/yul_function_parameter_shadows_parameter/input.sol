// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // A Yul function parameter may not shadow a *local* Solidity declaration,
    // including the enclosing function's parameter. solc accepts the
    // declaration but rejects the reference to `foo` from inside the Yul
    // function ("Cannot access local Solidity variables..."); we reject the
    // declaration outright (a conservative approximation).
    function f(uint256 foo) public pure {
        assembly {
            function g(foo) {
                pop(foo)
            }
        }
    }
}
