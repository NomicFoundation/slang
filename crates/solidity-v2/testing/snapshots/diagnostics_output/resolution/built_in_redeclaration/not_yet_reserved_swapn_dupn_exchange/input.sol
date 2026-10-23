// SPDX-License-Identifier: MIT
pragma solidity *;

// EIP-8024 names opcodes `swapn`, `dupn` and `exchange` in Amsterdam, but they
// are not Yul built-ins, so the names are still free to declare even when
// Amsterdam is targeted.
contract C {
    function f() public pure {
        assembly {
            function swapn(dupn) -> exchange {}
            let dupn
        }
    }

    function g() public pure {
        assembly {
            function dupn(exchange) -> swapn {}
            let exchange
        }
    }

    function h() public pure {
        assembly {
            function exchange(swapn) -> dupn {}
            let swapn
        }
    }
}
