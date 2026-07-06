// --- path: main.sol
import {A} from "lib.sol";
import {B as C} from "lib.sol";

contract Test {
    uint256[A] direct_import;
    uint256[C] aliased_import;
    uint256[A + C] combined;
}

// --- path: lib.sol
uint256 constant A = 3;
uint256 constant B = 5;
