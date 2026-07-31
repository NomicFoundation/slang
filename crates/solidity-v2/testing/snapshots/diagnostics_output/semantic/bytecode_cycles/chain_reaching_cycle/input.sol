// SPDX-License-Identifier: MIT
pragma solidity *;

// A is not part of the B and C cycle but reaches it, so it also reports.

contract A {
    B x = new B();
}

contract B {
    C x = new C();
}

contract C {
    B x = new B();
}
