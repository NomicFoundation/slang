// --- path: main.sol
import {Foo} from "lib/foo.sol";
import {Bar as LocalBar} from "lib/bar.sol";

contract Sample {
    function test() returns (int) {
        return Foo.foo() + LocalBar.bar();
    }
}

// --- path: lib/foo.sol
library Foo {
    function foo() returns (int) {
        return 1;
    }
}

// --- path: lib/bar.sol
library Bar {
    function bar() returns (int) {
        return 2;
    }
}
