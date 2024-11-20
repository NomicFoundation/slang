// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

contract $BuiltIns$ {
  function addmod(uint x, uint y, uint k) public returns (uint);
  function assert(bool condition) public;
  function blockhash(uint blockNumber) public returns (bytes32);
  function ecrecover(bytes32 hash, uint8 v, bytes32 r, bytes32 s) public returns (address);
  function gasleft() public returns (uint256);
  function keccak256(bytes memory) public returns (bytes32);
  function mulmod(uint x, uint y, uint k) public returns (uint);
  function require(bool condition) public;
  function require(bool condition, string memory message) public;
  function revert() public;
  function revert(string memory reason) public;
  function ripemd160(bytes memory) public returns (bytes20);
  function selfdestruct(address payable recipient) public;
  function sha256(bytes memory) public returns (bytes32);
  struct $AbiType {
    function(bytes memory, $Types) returns ($Types) decode;
    function($Types) returns (bytes memory) encode;
    function($Args) returns (bytes memory) encodePacked;
    function(bytes4 selector, $Args) returns (bytes memory) encodeWithSelector;
    function(string memory, $Args) returns (bytes memory) encodeWithSignature;
  }
  struct $address {
    uint256 balance;
    bytes code;
    bytes32 codehash;
    function(bytes memory) returns (bool, bytes memory) call;
    function(bytes memory) returns (bool, bytes memory) delegatecall;
    function(uint256) returns (bool) send;
    function(bytes memory) returns (bool, bytes memory) staticcall;
    function(uint256) transfer;
  }
  struct $Array {
    uint length;
    function() returns ($ValueType) push;
    function($ValueType) push;
    function() pop;
  }
  struct $FixedArray {
    uint length;
  }
  struct $BlockType {
    uint basefee;
    uint chainid;
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
    function($Args) returns (bytes memory) concat;
  }
  struct $CallOptions {
    uint gas;
    uint salt;
    uint value;
  }
  struct $ErrorType {
    bytes4 selector;
  }
  struct $Function {
  }
  struct $ExternalFunction {
    address address;
    bytes4 selector;
  }
  struct $MessageType {
    bytes data;
    address sender;
    bytes4 sig;
    uint value;
  }
  struct $StringType {
    function($Args) returns (string memory) concat;
  }
  struct $TransactionType {
    uint gasprice;
    address origin;
  }
  struct $ContractTypeType {
    string name;
    bytes creationCode;
    bytes runtimeCode;
    bytes4 interfaceId;
  }
  struct $InterfaceTypeType {
    string name;
    bytes4 interfaceId;
  }
  struct $IntTypeType {
    int min;
    int max;
  }
  struct $UserDefinedValueType {
    function($WrappedType) returns ($UserType) wrap;
    function($UserType) returns ($WrappedType) unwrap;
  }
  struct $YulExternal {
    uint slot;
    uint offset;
    uint length;
  }
  $Function $placeholder;
  $AbiType abi;
  $BlockType block;
  $BytesType $bytes;
  $MessageType msg;
  $StringType $string;
  $TransactionType tx;
}
