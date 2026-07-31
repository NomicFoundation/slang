// SPDX-License-Identifier: MIT
pragma solidity *;

// Mutual creation through state variable initializers, which are part of the
// creation bytecode.

contract A {
    B b = new B();
}

contract B {
    A a = new A();
}
