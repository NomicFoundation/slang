// ---- path: main.sol
import {foo} from "other.sol";

library Lib {
    function nop(uint x) public {}
}

using Lib for uint;

contract Test {
    function test(uint a) public {
        a.nop();
        foo(a).nop();
    }
}

function bar(uint a) {
    a.nop();
    foo(a).nop();
}

// ---- path: other.sol
function foo(uint a) pure returns (uint) {
    return a;
}
