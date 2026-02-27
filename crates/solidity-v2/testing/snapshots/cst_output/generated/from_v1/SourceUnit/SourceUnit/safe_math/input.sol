// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

// >>> Copied from crates/solidity/testing/snapshots/cst_output/SourceUnit/safe_math/input.sol
library SafeMath {
  function tryAdd(uint256 a, uint256 b) internal pure returns (bool, uint256) {
    unchecked {
      uint256 c = a + b;
      if (c < a) return (false, 0);
      return (true, c);
    }
  }
}

// <<<
