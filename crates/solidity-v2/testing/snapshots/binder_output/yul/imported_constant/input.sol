// --- path: main.sol
pragma solidity *;

import "./constants.sol";

contract Test {
    function test() public {
        assembly {
            let x := add(FOO, 1)
        }
    }
}

// --- path: constants.sol
pragma solidity *;

uint256 constant FOO = 1;
