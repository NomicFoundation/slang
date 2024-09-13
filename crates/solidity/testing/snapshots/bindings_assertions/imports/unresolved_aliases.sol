// --- path: main.sol
import {Foo} from "lib/foo.sol";
//      ^def:4
//      ^ref:!
import {Bar as LocalBar} from "lib/bar.sol";
//      ^ref:!
//             ^def:3

contract Sample {
    function test() returns (int) {
        return Foo.foo() + LocalBar.bar();
        //                          ^ref:!
        //                 ^ref:3
        //         ^ref:!
        //     ^ref:4
    }
}
