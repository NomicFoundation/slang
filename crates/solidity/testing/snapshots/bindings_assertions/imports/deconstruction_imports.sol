// --- path: main.sol
import {Foo} from "lib/foo.sol";
//      ^def:4
//      ^ref:6
import {Bar as LocalBar} from "lib/bar.sol";
//      ^ref:5
//             ^def:3

contract Sample {
    function test() returns (int) {
        return Foo.foo() + LocalBar.bar();
        //                          ^ref:2
        //                 ^ref:5
        //         ^ref:1
        //     ^ref:6
    }
}

// --- path: lib/foo.sol
library Foo {
    //  ^def:6
    function foo() returns (int) {
        //   ^def:1
        return 1;
    }
}

// --- path: lib/bar.sol
library Bar {
    //  ^def:5
    function bar() returns (int) {
        //   ^def:2
        return 2;
    }
}
