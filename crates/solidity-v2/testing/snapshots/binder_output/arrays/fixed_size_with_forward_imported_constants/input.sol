// --- path: main.sol
// The imported file sorts after the importer, so it is typed later. The
// sizes still fold: constant evaluation does not depend on typing order.
import {A} from "z_lib.sol";
import {B as C} from "z_lib.sol";

contract Test {
    uint256[A] direct_import;
    uint256[C] aliased_import;
}

// --- path: z_lib.sol
uint256 constant A = 3;
uint256 constant B = 5;
