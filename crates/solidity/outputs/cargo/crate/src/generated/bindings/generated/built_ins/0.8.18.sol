// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

contract $SolidityBuiltIns$ {
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
    function(bytes memory encodedData, $Type[] encodedTypesTuple) returns ($Any[]) decode;
    function($Any[] valuesToEncode) returns (bytes memory) encode;
    function(function() functionPointer, $Any[] functionArgumentsTuple) returns (bytes memory) encodeCall;
    function($Any[] valuesToEncode) returns (bytes memory) encodePacked;
    function(bytes4 selector, $Any[] functionArgumentsTuple) returns (bytes memory) encodeWithSelector;
    function(string memory signature, $Any[] valuesToEncode) returns (bytes memory) encodeWithSignature;
  }
  struct $address {
    uint256 balance;
    bytes code;
    bytes32 codehash;
    function(bytes memory) returns (bool, bytes memory) call;
    function(bytes memory) returns (bool, bytes memory) delegatecall;
    function(uint256 amount) returns (bool) send;
    function(bytes memory) returns (bool, bytes memory) staticcall;
    function(uint256 amount) transfer;
  }
  struct $Array {
    uint length;
    function() returns ($ValueType) push;
    function($ValueType element) push;
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
    uint prevrandao;
    uint timestamp;
  }
  struct $bytes {
    uint length;
  }
  struct $BytesType {
    function(bytes[] bytesToConcatenate) returns (bytes memory) concat;
  }
  struct $CallOptions {
    uint gas;
    uint salt;
    uint value;
  }
  struct Error {
    string reason;
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
  struct Panic {
    uint errorCode;
  }
  struct $StringType {
    function(string[] stringsToConcatenate) returns (string memory) concat;
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
    function($WrappedType elementaryType) returns ($UserType) wrap;
    function($UserType userType) returns ($WrappedType) unwrap;
  }
  $Function $placeholder;
  $AbiType abi;
  $BlockType block;
  $BytesType $bytes;
  $MessageType msg;
  $StringType $string;
  $TransactionType tx;
}
contract $YulBuiltIns$ {
  struct $YulExternal {
    uint256 slot;
    uint256 offset;
    uint256 length;
  }
  function add(uint256 x, uint256 y) public returns (uint256);
  function addmod(uint256 x, uint256 y, uint256 m) public returns (uint256);
  function address() public returns (uint256);
  function and(uint256 x, uint256 y) public returns (uint256);
  function balance(uint256 a) public returns (uint256);
  function blockhash(uint256 b) public returns (uint256);
  function callcode(uint256 g, uint256 a, uint256 v, uint256 in_, uint256 insize, uint256 out, uint256 outsize) public returns (uint256);
  function calldatacopy(uint256 t, uint256 f, uint256 s) public;
  function calldataload(uint256 p) public returns (uint256);
  function calldatasize() public returns (uint256);
  function caller() public returns (uint256);
  function call(uint256 g, uint256 a, uint256 v, uint256 in_, uint256 insize, uint256 out, uint256 outsize) public returns (uint256);
  function callvalue() public returns (uint256);
  function coinbase() public returns (uint256);
  function create(uint256 v, uint256 p, uint256 n) public returns (uint256);
  function delegatecall(uint256 g, uint256 a, uint256 in_, uint256 insize, uint256 out, uint256 outsize) public returns (uint256);
  function div(uint256 x, uint256 y) public returns (uint256);
  function eq(uint256 x, uint256 y) public returns (uint256);
  function exp(uint256 x, uint256 y) public returns (uint256);
  function extcodecopy(uint256 a, uint256 t, uint256 f, uint256 s) public;
  function extcodesize(uint256 a) public returns (uint256);
  function gas() public returns (uint256);
  function gaslimit() public returns (uint256);
  function gasprice() public returns (uint256);
  function gt(uint256 x, uint256 y) public returns (uint256);
  function invalid() public;
  function iszero(uint256 x) public returns (uint256);
  function log0(uint256 p, uint256 s) public;
  function log1(uint256 p, uint256 s, uint256 t1) public;
  function log2(uint256 p, uint256 s, uint256 t1, uint256 t2) public;
  function log3(uint256 p, uint256 s, uint256 t1, uint256 t2, uint256 t3) public;
  function log4(uint256 p, uint256 s, uint256 t1, uint256 t2, uint256 t3) public;
  function lt(uint256 x, uint256 y) public returns (uint256);
  function mload(uint256 p) public returns (uint256);
  function mod(uint256 x, uint256 y) public returns (uint256);
  function msize() public returns (uint256);
  function mstore8(uint256 p, uint256 v) public;
  function mstore(uint256 p, uint256 v) public;
  function mul(uint256 x, uint256 y) public returns (uint256);
  function mulmod(uint256 x, uint256 y, uint256 m) public returns (uint256);
  function not(uint256 x) public returns (uint256);
  function number(uint256 x) public returns (uint256);
  function origin() public returns (uint256);
  function or(uint256 x, uint256 y) public returns (uint256);
  function pop(uint256 x) public returns (uint256);
  function revert(uint256 p, uint256 s) public;
  function sdiv(uint256 x, uint256 y) public returns (uint256);
  function selfdestruct(uint256 a) public;
  function sgt(uint256 x, uint256 y) public returns (uint256);
  function signextend(uint256 i, uint256 x) public returns (uint256);
  function sload(uint256 p) public returns (uint256);
  function slt(uint256 x, uint256 y) public returns (uint256);
  function smod(uint256 x, uint256 y) public returns (uint256);
  function sstore(uint256 p, uint256 v) public;
  function stop() public;
  function sub(uint256 x, uint256 y) public returns (uint256);
  function timestamp() public returns (uint256);
  function xor(uint256 x, uint256 y) public returns (uint256);
  function keccak256(uint256 p, uint256 n) public returns (uint256);
  function returndatacopy(uint256 t, uint256 f, uint256 s) public;
  function returndatasize() public returns (uint256);
  function staticcall(uint256 g, uint256 a, uint256 in_, uint256 insize, uint256 out, uint256 outsize) public returns (uint256);
  function create2(uint256 v, uint256 p, uint256 n, uint256 s) public returns (uint256);
  function extcodehash(uint256 a) public returns (uint256);
  function sar(uint256 x, uint256 y) public returns (uint256);
  function shl(uint256 x, uint256 y) public returns (uint256);
  function shr(uint256 x, uint256 y) public returns (uint256);
  function chainid() public returns (uint256);
  function selfbalance() public returns (uint256);
  function basefee() public returns (uint256);
  function prevrandao() public returns (uint256);
}
