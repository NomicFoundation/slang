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
  struct $AbiType {
  }
  struct $address {
    uint256 balance;
    function(bytes memory) returns (bool) call;
    function(bytes memory) returns (bool, bytes memory) callcode;
    function(bytes memory) returns (bool) delegatecall;
    function(uint256) returns (bool) send;
    function(uint256) transfer;
  }
  struct $Array {
    uint length;
    function($ValueType) returns (uint) push;
    function() pop;
  }
  struct $FixedArray {
    uint length;
  }
  struct $BlockType {
    address payable coinbase;
    uint difficulty;
    uint gaslimit;
    uint number;
    uint timestamp;
    function(uint) returns (bytes32) blockhash;
  }
  struct $bytes {
    uint length;
  }
  struct $BytesType {
    function($Args) returns (bytes memory) concat;
  }
  struct $Function {
    function(uint) returns (function()) gas;
    function(uint) returns (function()) value;
  }
  struct $ExternalFunction {
    function(uint) returns (function()) gas;
    function(uint) returns (function()) value;
  }
  struct $MessageType {
    bytes data;
    uint256 gas;
    address payable sender;
    bytes4 sig;
    uint value;
  }
  struct $StringType {
    function($Args) returns (string memory) concat;
  }
  struct $TransactionType {
    uint gasprice;
    address payable origin;
  }
  struct $ContractTypeType {
    string name;
  }
  struct $InterfaceTypeType {
    string name;
  }
  struct $IntTypeType {
  }
  $Function $placeholder;
  $AbiType abi;
  $BlockType block;
  $BytesType $bytes;
  $MessageType msg;
  uint now;
  $StringType $string;
  $TransactionType tx;
}
