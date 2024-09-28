// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

contract $BuiltIns$ {
  function addmod(uint x, uint y, uint k) public returns (uint);
  function assert(bool condition) public;
  function require(bool condition) public;
  function require(bool condition, string memory message) public;
  function revert(string memory reason) public;
  struct $address {
    uint256 balance;
    function(uint256 amount) returns (bool) send;
  }
  struct $type {
    string name;
    bytes creationCode;
    bytes runtimeCode;
    bytes4 interfaceId;
    int min;
    int max;
  }
  struct $txType {
    uint gasprice;
    address payable origin;
  }
  uint now;
  $txType tx;
}
