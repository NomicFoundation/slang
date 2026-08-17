// ---- path: main.sol
import "./lib.sol" as A;
import "./lib2.sol" as B;

function pick() pure returns (uint256) {
    return (true ? A : B).K;
}

// ---- path: lib.sol
uint256 constant K = 1;

// ---- path: lib2.sol
uint256 constant K = 2;
