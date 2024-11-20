// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

contract $BuiltIns$ {
  function addmod(uint x, uint y, uint k) public returns (uint);
  function assert(bool condition) public;
  function blockhash(uint blockNumber) public returns (bytes32);
  function ecrecover(bytes32 hash, uint8 v, bytes32 r, bytes32 s) public returns (address);
  function gasleft() public returns (uint256);
  function keccak256(bytes memory) public returns (bytes32);
  function log0(bytes32) public;
  function log1(bytes32, bytes32) public;
  function log2(bytes32, bytes32, bytes32) public;
  function log3(bytes32, bytes32, bytes32, bytes32) public;
  function log4(bytes32, bytes32, bytes32, bytes32, bytes32) public;
  function mulmod(uint x, uint y, uint k) public returns (uint);
  function require(bool condition) public;
  function require(bool condition, string memory message) public;
  function revert() public;
  function revert(string memory reason) public;
  function ripemd160(bytes memory) public returns (bytes20);
  function selfdestruct(address payable recipient) public;
  function sha256(bytes memory) public returns (bytes32);
  struct $abiType {
    function(bytes memory, $args) returns ($args) decode;
    function($args) returns (bytes memory) encode;
    function($args) returns (bytes memory) encodePacked;
    function(bytes4 selector, $args) returns (bytes memory) encodeWithSelector;
    function(string memory, $args) returns (bytes memory) encodeWithSignature;
  }
  struct $address {
    uint256 balance;
    function(bytes memory) returns (bool, bytes memory) call;
    function(bytes memory) returns (bool, bytes memory) delegatecall;
    function(uint256) returns (bool) send;
    function(bytes memory) returns (bool, bytes memory) staticcall;
    function(uint256) transfer;
  }
  struct $array {
    uint length;
    function(uint) returns ($element) $index;
    function($element) returns (uint) push;
    function() pop;
  }
  struct $arrayFixed {
    uint length;
    function(uint) returns ($element) $index;
  }
  struct $blockType {
    address payable coinbase;
    uint difficulty;
    uint gaslimit;
    uint number;
    uint timestamp;
  }
  struct $bytes {
    function($args) returns (bytes memory) concat;
  }
  struct $functionExternal {
    $selector selector;
    function(uint) returns ($function) gas;
    function(uint) returns ($function) value;
  }
  struct $msgType {
    bytes data;
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
  struct $typeContractType {
    string name;
  }
  struct $typeInterfaceType {
    string name;
  }
  struct $typeIntType {
  }
  $function $placeholder;
  $abiType abi;
  $blockType block;
  $msgType msg;
  uint now;
  $txType tx;
}
