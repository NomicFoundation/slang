// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    constructor() {}
}

contract D {
    function createC() public returns (C) {
        // https://github.com/argotorg/solidity/blob/v0.8.0/test/libsolidity/syntaxTests/functionTypes/error_deprecate_value_constructor_nonpayable.sol
        // `new C` has no `value` member when the constructor of `C` is not payable.
        C c = (new C).value(2)();
        return c;
    }
}
