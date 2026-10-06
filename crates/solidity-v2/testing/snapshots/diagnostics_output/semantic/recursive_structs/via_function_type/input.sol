// SPDX-License-Identifier: MIT
pragma solidity *;

// A function type holds no value of its parameter types, so a struct with a
// function taking its own type is finite and accepted.

contract Test {
    struct MyStruct {
        function (MyStruct memory) internal f;
    }
}
