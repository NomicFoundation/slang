// --- path: a.sol
// The two files import each other, so no topological order exists. Both
// sizes still fold, since evaluation does not depend on the file processing
// order. solc instead cuts the cycle in file-name order and rejects the
// reference crossing the cut edge (here: `N` used in b.sol).
import {K} from "b.sol";

uint256 constant N = 3;

contract A {
    uint256[K] x;
}

// --- path: b.sol
import {N} from "a.sol";

uint256 constant K = 5;

contract B {
    uint256[N] y;
}
