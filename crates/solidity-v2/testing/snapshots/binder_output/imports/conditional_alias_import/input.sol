// ---- path: main.sol
import "./lib.sol" as A;
import "./lib.sol" as B;

function pick() pure returns (uint256) {
    return (true ? A : B).K;
}

// ---- path: lib.sol
uint256 constant K = 1;
