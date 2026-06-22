// ---- path: main.sol
import {g as f} from "other.sol";

using {f} for uint;

contract Test {
    function test(uint a) public {
        a.f();
    }
}

// ---- path: other.sol
function g(uint a) {}
