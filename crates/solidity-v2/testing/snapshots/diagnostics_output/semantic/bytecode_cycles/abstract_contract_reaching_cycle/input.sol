// SPDX-License-Identifier: MIT
pragma solidity *;

// A is abstract, so it is never deployed and nothing can embed its bytecode.
// Its initializer still reaches the B and C cycle, so it reports like any
// other contract.

abstract contract A {
    B x = new B();
}

contract B {
    C x = new C();
}

contract C {
    B x = new B();
}
