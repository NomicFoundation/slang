// ---- path: base.sol
pragma solidity *;

contract Base {
    constructor(uint256 x) {}
}

// ---- path: main.sol
pragma solidity *;

import "base.sol" as M;

contract Derived is M.Base {
    constructor() M.Base(1) {}
}
