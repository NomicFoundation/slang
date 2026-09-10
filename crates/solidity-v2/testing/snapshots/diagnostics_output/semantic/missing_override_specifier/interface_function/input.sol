// SPDX-License-Identifier: MIT
pragma solidity *;

interface I {
    function f() external;
}

// A function overriding only an interface function doesn't need the
// `override` specifier from 0.8.8 on. Before that it is required.
contract C is I {
    function f() external {}
}
