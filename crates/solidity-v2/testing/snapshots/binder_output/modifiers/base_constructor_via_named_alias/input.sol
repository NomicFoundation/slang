// ---- path: base.sol
pragma solidity *;

contract Base {
    constructor(uint256 x) {}
}

// ---- path: main.sol
pragma solidity *;

import {Base as Aliased} from "base.sol";

contract Derived is Aliased {
    constructor() Aliased(1) {}
}
