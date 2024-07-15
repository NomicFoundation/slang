// --- path: main.sol
import * as foo from "lib/foo.sol";
//          ^def:1

contract Example {
    function test(int x) returns (int) {
        return foo.Foo.test(x);
        //             ^ref:3
        //         ^ref:2
        //     ^ref:1
    }
}

// --- path: lib/foo.sol
library Foo {
    //  ^def:2
    function test(int x) returns (int) {
        //   ^def:3
        return x + 2;
    }
}
