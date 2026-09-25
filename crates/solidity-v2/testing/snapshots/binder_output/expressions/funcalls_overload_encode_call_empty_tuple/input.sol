pragma solidity *;

interface I {
  function m() external;
}

contract Test {
  function f() private pure returns (uint256) {}

  function f(uint256 a, bytes memory data) private pure returns (uint256) {}

  function test() internal pure returns (uint256) {
    return f(1, abi.encodeCall(I.m, ()));
  }
}
