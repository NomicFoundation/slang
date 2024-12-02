// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

contract $SolidityBuiltIns$ {
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
  struct $AbiType {
    function(bytes memory encodedData, $Type[] encodedTypesTuple) returns ($Any[]) decode;
    function($Any[] valuesToEncode) returns (bytes memory) encode;
    function($Any[] valuesToEncode) returns (bytes memory) encodePacked;
    function(bytes4 selector, $Any[] functionArgumentsTuple) returns (bytes memory) encodeWithSelector;
    function(string memory signature, $Any[] valuesToEncode) returns (bytes memory) encodeWithSignature;
  }
  struct $address {
    uint256 balance;
    function(bytes memory) returns (bool, bytes memory) call;
    function(bytes memory) returns (bool, bytes memory) delegatecall;
    function(uint256 amount) returns (bool) send;
    function(bytes memory) returns (bool, bytes memory) staticcall;
    function(uint256 amount) transfer;
  }
  struct $Array {
    uint length;
    function($ValueType element) returns (uint) push;
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
  }
  struct $bytes {
    uint length;
  }
  struct $BytesType {
    function(bytes[] bytesToConcatenate) returns (bytes memory) concat;
  }
  struct $Function {
    function(uint amount) returns (function()) gas;
    function(uint amount) returns (function()) value;
  }
  struct $ExternalFunction {
    bytes4 selector;
    function(uint amount) returns (function()) gas;
    function(uint amount) returns (function()) value;
  }
  struct $MessageType {
    bytes data;
    address payable sender;
    bytes4 sig;
    uint value;
  }
  struct $StringType {
    function(string[] stringsToConcatenate) returns (string memory) concat;
  }
  struct $TransactionType {
    uint gasprice;
    address payable origin;
  }
  struct $ContractTypeType {
    string name;
    bytes creationCode;
    bytes runtimeCode;
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
contract $YulBuiltIns$ {
  struct $YulExternal {
    uint slot;
    uint offset;
    uint length;
  }
}
