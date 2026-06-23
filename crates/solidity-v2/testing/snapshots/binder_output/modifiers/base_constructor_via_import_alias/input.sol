// ---- path: base.sol
contract Base {
    constructor(uint256 x) {}
}

// ---- path: main.sol
import "base.sol" as M;

contract Derived is M.Base {
    constructor() M.Base(1) {}
}
