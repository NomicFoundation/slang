// This file is generated automatically. Do not edit by hand.
contract $BuiltIns$ {
  function addmod(uint x, uint y, uint k) public returns (uint);
  function assert(bool condition) public;
  function require(bool condition) public;
  function require(bool condition, string memory message) public;
  function revert(string memory reason) public;
  struct $BuiltIn$Address {
    uint256 balance;
    function(uint256 amount) returns (bool) send;
  }
  struct $BuiltIn$TxType {
    uint gasprice;
    address payable origin;
  }
  uint now;
  $BuiltIn$TxType tx;
}
