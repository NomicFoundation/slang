pragma solidity *;

library L {
  function f(uint256[] calldata, uint256) public pure returns (uint256) { return 1; }
  function f(uint256[] calldata, string memory) public pure returns (uint256) { return 2; }
}

contract C {
  using L for uint256[];

  function g(uint256[] memory a) internal pure {
    a.f(1);
  }
}
