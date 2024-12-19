// --- path: main.sol
import * as foo from "lib/foo.sol";

contract Example {
    function test(int x) returns (int) {
        return foo.Foo.test(x);
    }
}

// --- path: lib/foo.sol
library Foo {
    function test(int x) returns (int) {
        return x + 2;
    }
}
