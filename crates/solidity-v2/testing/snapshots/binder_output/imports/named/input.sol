// --- path: main.sol
pragma solidity *;

import * as foo from "lib/foo.sol";

contract Example {
    function test(int x) public returns (int) {
        return foo.Foo.test(x);
    }
}

// --- path: lib/foo.sol
pragma solidity *;

library Foo {
    function test(int x) public returns (int) {
        return x + 2;
    }
}
