// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

contract $BuiltIns$ {
  function addmod(uint x, uint y, uint k) public returns (uint);
  function assert(bool condition) public;
  function blockhash(uint blockNumber) public returns (bytes32);
  function blobhash(uint index) public returns (bytes32);
  function ecrecover(bytes32 hash, uint8 v, bytes32 r, bytes32 s) public returns (address);
  function gasleft() public returns (uint256);
  function keccak256(bytes memory) public returns (bytes32);
  function mulmod(uint x, uint y, uint k) public returns (uint);
  function revert() public;
  function revert(string memory reason) public;
  function ripemd160(bytes memory) public returns (bytes20);
  function selfdestruct(address payable recipient) public;
  function sha256(bytes memory) public returns (bytes32);
  function sha3(bytes memory) public returns (bytes32);
  function suicide(address payable recipient) public;
  struct $abiType {
    function(bytes memory, $args) returns ($args) decode;
    function($args) returns (bytes memory) encode;
    function($args) returns (bytes memory) encodePacked;
    function(bytes4 selector, $args) returns (bytes memory) encodeWithSelector;
    function(string memory, $args) returns (bytes memory) encodeWithSignature;
    function(function(), $args) returns (bytes memory) encodeCall;
  }
  struct $address {
    uint256 balance;
    bytes32 codehash;
    function(bytes memory) returns (bool) call;
    function(bytes memory) returns (bool, bytes memory) callcode;
    function(bytes memory) returns (bool) delegatecall;
    function(uint256) returns (bool) send;
    function(bytes memory) returns (bool) staticcall;
    function(uint256) transfer;
  }
  struct $blockType {
    uint basefee;
    uint blobbasefee;
    uint chainid;
    address payable coinbase;
    uint difficulty;
    uint gaslimit;
    uint number;
    uint prevrandao;
    uint timestamp;
    function(uint) returns (bytes32) blockhash;
  }
  struct $bytes {
    function($args) returns (bytes memory) concat;
  }
  struct $msgType {
    bytes data;
    address sender;
    bytes4 sig;
    uint value;
    function() returns (uint256) gas;
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
    bytes creationCode;
    bytes runtimeCode;
    bytes4 interfaceId;
    int min;
    int max;
  }
  $abiType abi;
  $blockType block;
  $msgType msg;
  uint now;
  $SuperType super;
  $ThisType this;
  $txType tx;
}
