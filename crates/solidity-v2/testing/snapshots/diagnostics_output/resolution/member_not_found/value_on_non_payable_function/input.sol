// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public view returns (C) {
        return this;
    }

    function g() public {
        // https://github.com/argotorg/solidity/blob/v0.8.0/test/libsolidity/syntaxTests/functionTypes/call_value_on_non_constructor.sol
        // `value` is only a member of payable functions.
        this.f.value();
    }
}
