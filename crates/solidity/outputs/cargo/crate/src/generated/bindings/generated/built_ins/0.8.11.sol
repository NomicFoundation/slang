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
  struct $abiType {
    function(bytes memory, $args) returns ($args) decode;
    function($args) returns (bytes memory) encode;
    function(function(), $args) returns (bytes memory) encodeCall;
    function($args) returns (bytes memory) encodePacked;
    function(bytes4 selector, $args) returns (bytes memory) encodeWithSelector;
    function(string memory, $args) returns (bytes memory) encodeWithSignature;
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
  struct $array {
    uint length;
    function() returns ($element) push;
    function($element) push;
    function() pop;
  }
  struct $arrayFixed {
    uint length;
  }
  struct $blockType {
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
  struct $bytesType {
    function($args) returns (bytes memory) concat;
  }
  struct $callOptions {
    uint gas;
    uint salt;
    uint value;
  }
  struct $errorType {
    bytes4 selector;
  }
  struct $function {
  }
  struct $functionExternal {
    $address address;
    $selector selector;
  }
  struct $msgType {
    bytes data;
    address sender;
    bytes4 sig;
    uint value;
  }
  struct $stringType {
    function($args) returns (string memory) concat;
  }
  struct $txType {
    uint gasprice;
    address origin;
  }
  struct $typeContractType {
    string name;
    bytes creationCode;
    bytes runtimeCode;
    bytes4 interfaceId;
  }
  struct $typeInterfaceType {
    string name;
    bytes4 interfaceId;
  }
  struct $typeIntType {
    int min;
    int max;
  }
  struct $userTypeType {
    function($elementaryType) returns ($userType) wrap;
    function($userType) returns ($elementaryType) unwrap;
  }
  $function $placeholder;
  $abiType abi;
  $blockType block;
  $bytesType $bytes;
  $msgType msg;
  $stringType $string;
  $txType tx;
}
