// ---- path: main.sol
import "./lib.sol";

contract Test {
    using AmountLib for Amount;

    function test() public pure {
        Amount x;
        x.nop().min();
        x.min();
    }
}

// ---- path: lib.sol
type Amount is uint64;

using {min} for Amount global;

function min(Amount x) pure returns (Amount) {
    return x;
}

library AmountLib {
    function nop(Amount x) internal pure returns (Amount) {
        return x;
    }
}
