// SPDX-License-Identifier: MIT
pragma solidity *;

// Recovered from solc:
// test/libsolidity/syntaxTests/functionCalls/enum_value_not_callable.sol
// Calling an enum member, which is a value and not callable.

enum E {
    A,
    B,
    C
}

contract C {
    // TypeError 5704: This expression is not callable.
    uint256 a = E.B(1000);

    // A cast through the enum type name itself is fine.
    E b = E(1);
}
