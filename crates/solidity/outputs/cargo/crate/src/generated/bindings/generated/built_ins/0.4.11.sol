// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

contract $BuiltIns$ {
  function addmod(uint x, uint y, uint k) public returns (uint);
  function assert(bool condition) public;
  function ecrecover(bytes32 hash, uint8 v, bytes32 r, bytes32 s) public returns (address);
  function keccak256(bytes memory) public returns (bytes32);
  function log0(bytes32) public;
  function log1(bytes32, bytes32) public;
  function log2(bytes32, bytes32, bytes32) public;
  function log3(bytes32, bytes32, bytes32, bytes32) public;
  function log4(bytes32, bytes32, bytes32, bytes32, bytes32) public;
  function mulmod(uint x, uint y, uint k) public returns (uint);
  function require(bool condition) public;
  function revert() public;
  function ripemd160(bytes memory) public returns (bytes20);
  function selfdestruct(address payable recipient) public;
  function sha256(bytes memory) public returns (bytes32);
  function sha3(bytes memory) public returns (bytes32);
  function suicide(address payable recipient) public;
  struct $abiType {
  }
  struct $address {
    uint256 balance;
    function(bytes memory) returns (bool) call;
    function(bytes memory) returns (bool, bytes memory) callcode;
    function(bytes memory) returns (bool) delegatecall;
    function(uint256) returns (bool) send;
    function(uint256) transfer;
  }
  struct $array {
    uint length;
    function($element) returns (uint) push;
    function() pop;
  }
  struct $blockType {
    address payable coinbase;
    uint difficulty;
    uint gaslimit;
    uint number;
    uint timestamp;
    function(uint) returns (bytes32) blockhash;
  }
  struct $bytes {
    function($args) returns (bytes memory) concat;
  }
  struct $function {
    function(uint) returns ($function) gas;
    function(uint) returns ($function) value;
  }
  struct $msgType {
    bytes data;
    uint256 gas;
    address payable sender;
    bytes4 sig;
    uint value;
  }
  struct $string {
    function($args) returns (string memory) concat;
  }
  struct $txType {
    uint gasprice;
    address payable origin;
  }
  struct $type {
    string name;
  }
  $function $placeholder;
  $abiType abi;
  $blockType block;
  $msgType msg;
  uint now;
  $SuperType super;
  $ThisType this;
  $txType tx;
}
