// --- path: main.sol
import "./constants.sol";

contract Test {
    function test() public {
        assembly {
            let x := add(FOO, 1)
        }
    }
}

// --- path: constants.sol
uint256 constant FOO = 1;
