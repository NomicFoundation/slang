// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    function f() public pure {}
}

contract B {
    A public a;
}

contract C {
    B b;

    function f() public view {
        // https://github.com/argotorg/solidity/blob/v0.8.0/test/libsolidity/syntaxTests/nameAndTypeResolution/576_member_getter_call_without_parentheses.sol
        // TypeError 6005: Member "f" not found or not visible after argument-dependent lookup in function () view external returns (contract A). Did you intend to call the function?
        b.a.f();
    }
}
