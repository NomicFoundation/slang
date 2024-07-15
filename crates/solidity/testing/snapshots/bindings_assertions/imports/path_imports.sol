// --- path: main.sol
import "lib/foo.sol";
import "lib/bar.sol" as bar;
//                      ^def:3

contract Example {
    function test() returns (int) {
        return Foo.from_foo() + bar.Bar.from_bar();
        //                              ^ref:2
        //                      ^ref:3
        //         ^ref:1
    }
}

// --- path: lib/foo.sol
library Foo {
    function from_foo() returns (int) {
        //   ^def:1
        return 5;
    }
}

// --- path: lib/bar.sol
library Bar {
    function from_bar() returns (int) {
        //   ^def:2
        return 4;
    }
}
