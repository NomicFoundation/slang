// --- path: main.sol
pragma solidity *;

import {Foo} from "lib/foo.sol";
import {Bar as LocalBar} from "lib/bar.sol";

contract Sample {
    function test() public returns (int) {
        return Foo.foo() + LocalBar.bar();
    }
}

// --- path: lib/foo.sol
pragma solidity *;

library Foo {
    function foo() public returns (int) {
        return 1;
    }
}

// --- path: lib/bar.sol
pragma solidity *;

library Bar {
    function bar() public returns (int) {
        return 2;
    }
}
