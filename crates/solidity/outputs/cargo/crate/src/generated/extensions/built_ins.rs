// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use metaslang_bindings::{FileGraphBuilder, ScopeBuilder};
use metaslang_cst::kinds::KindTypes;
use semver::Version;
#[allow(clippy::too_many_lines)]

pub fn define_built_ins<KT: KindTypes + 'static>(
    builder: &mut FileGraphBuilder<'_, KT>,
    scope: &mut impl ScopeBuilder<KT>,
    version: &Version,
) {
    if *version < Version::new(0, 4, 12) {
        // 0.4.11
        // SolidityBuiltIns
        {
            scope.define_function(
                builder,
                "addmod",
                &["uint x", "uint y", "uint k"],
                Some("uint256"),
            );
            scope.define_function(builder, "assert", &["bool condition"], None);
            scope.define_function(
                builder,
                "ecrecover",
                &["bytes32 hash", "uint8 v", "bytes32 r", "bytes32 s"],
                Some("address"),
            );
            scope.define_function(builder, "keccak256", &["bytes memory"], Some("bytes32"));
            scope.define_function(builder, "log0", &["bytes32"], None);
            scope.define_function(builder, "log1", &["bytes32", "bytes32"], None);
            scope.define_function(builder, "log2", &["bytes32", "bytes32", "bytes32"], None);
            scope.define_function(
                builder,
                "log3",
                &["bytes32", "bytes32", "bytes32", "bytes32"],
                None,
            );
            scope.define_function(
                builder,
                "log4",
                &["bytes32", "bytes32", "bytes32", "bytes32", "bytes32"],
                None,
            );
            scope.define_function(
                builder,
                "mulmod",
                &["uint x", "uint y", "uint k"],
                Some("uint256"),
            );
            scope.define_function(builder, "require", &["bool condition"], None);
            scope.define_function(builder, "revert", &[], None);
            scope.define_function(builder, "ripemd160", &["bytes memory"], Some("bytes20"));
            scope.define_function(
                builder,
                "selfdestruct",
                &["address payable recipient"],
                None,
            );
            scope.define_function(builder, "sha256", &["bytes memory"], Some("bytes32"));
            scope.define_function(builder, "sha3", &["bytes memory"], Some("bytes32"));
            scope.define_function(builder, "suicide", &["address payable recipient"], None);
            _ = scope.define_type(builder, "%AbiType");
            let mut type_scope = scope.define_type(builder, "address");
            type_scope.define_field(builder, "balance", "uint256");
            type_scope.define_function(builder, "call", &["bytes memory"], Some("bool"));
            type_scope.define_function(
                builder,
                "callcode",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "delegatecall", &["bytes memory"], Some("bool"));
            type_scope.define_function(builder, "send", &["uint256 amount"], Some("bool"));
            type_scope.define_function(builder, "transfer", &["uint256 amount"], None);
            let mut type_scope = scope.define_type(builder, "%Array");
            type_scope.define_field(builder, "length", "uint256");
            type_scope.define_function(builder, "push", &["%ValueType element"], Some("uint256"));
            type_scope.define_function(builder, "pop", &[], None);
            let mut type_scope = scope.define_type(builder, "%FixedArray");
            type_scope.define_field(builder, "length", "uint256");
            let mut type_scope = scope.define_type(builder, "%BlockType");
            type_scope.define_field(builder, "coinbase", "address payable");
            type_scope.define_field(builder, "difficulty", "uint256");
            type_scope.define_field(builder, "gaslimit", "uint256");
            type_scope.define_field(builder, "number", "uint256");
            type_scope.define_field(builder, "timestamp", "uint256");
            type_scope.define_function(builder, "blockhash", &["uint"], Some("bytes32"));
            let mut type_scope = scope.define_type(builder, "bytes");
            type_scope.define_field(builder, "length", "uint256");
            let mut type_scope = scope.define_type(builder, "%BytesType");
            type_scope.define_function(
                builder,
                "concat",
                &["bytes[] bytesToConcatenate"],
                Some("bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "%Function");
            type_scope.define_function(builder, "gas", &["uint amount"], Some("function()"));
            type_scope.define_function(builder, "value", &["uint amount"], Some("function()"));
            let mut type_scope = scope.define_type(builder, "%ExternalFunction");
            type_scope.define_function(builder, "gas", &["uint amount"], Some("function()"));
            type_scope.define_function(builder, "value", &["uint amount"], Some("function()"));
            let mut type_scope = scope.define_type(builder, "%MessageType");
            type_scope.define_field(builder, "data", "bytes");
            type_scope.define_field(builder, "gas", "uint256");
            type_scope.define_field(builder, "sender", "address payable");
            type_scope.define_field(builder, "sig", "bytes4");
            type_scope.define_field(builder, "value", "uint256");
            let mut type_scope = scope.define_type(builder, "%StringType");
            type_scope.define_function(
                builder,
                "concat",
                &["string[] stringsToConcatenate"],
                Some("string memory"),
            );
            let mut type_scope = scope.define_type(builder, "%TransactionType");
            type_scope.define_field(builder, "gasprice", "uint256");
            type_scope.define_field(builder, "origin", "address payable");
            let mut type_scope = scope.define_type(builder, "%ContractTypeType");
            type_scope.define_field(builder, "name", "string");
            let mut type_scope = scope.define_type(builder, "%InterfaceTypeType");
            type_scope.define_field(builder, "name", "string");
            _ = scope.define_type(builder, "%IntTypeType");
            scope.define_field(builder, "%_", "%Function");
            scope.define_field(builder, "abi", "%AbiType");
            scope.define_field(builder, "block", "%BlockType");
            scope.define_field(builder, "bytes", "%BytesType");
            scope.define_field(builder, "msg", "%MessageType");
            scope.define_field(builder, "now", "uint256");
            scope.define_field(builder, "string", "%StringType");
            scope.define_field(builder, "tx", "%TransactionType");
        }
        // YulBuiltIns
        {
            let mut scope = scope.new_context(builder, "@yul");
            let mut type_scope = scope.define_type(builder, "%YulExternal");
            type_scope.define_field(builder, "slot", "uint256");
            type_scope.define_field(builder, "offset", "uint256");
            type_scope.define_field(builder, "length", "uint256");
            scope.define_function(builder, "add", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "addmod",
                &["uint256 x", "uint256 y", "uint256 m"],
                Some("uint256"),
            );
            scope.define_function(builder, "address", &[], Some("uint256"));
            scope.define_function(builder, "and", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "balance", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "blockhash", &["uint256 b"], Some("uint256"));
            scope.define_function(
                builder,
                "byte",
                &["uint256 n", "uint256 x"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "callcode",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 v",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "calldatacopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "calldataload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "calldatasize", &[], Some("uint256"));
            scope.define_function(builder, "caller", &[], Some("uint256"));
            scope.define_function(
                builder,
                "call",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 v",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(builder, "callvalue", &[], Some("uint256"));
            scope.define_function(
                builder,
                "codecopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "codesize", &[], Some("uint256"));
            scope.define_function(builder, "coinbase", &[], Some("uint256"));
            scope.define_function(
                builder,
                "create",
                &["uint256 v", "uint256 p", "uint256 n"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "delegatecall",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(builder, "div", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "eq", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "exp", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "extcodecopy",
                &["uint256 a", "uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "extcodesize", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "gas", &[], Some("uint256"));
            scope.define_function(builder, "gaslimit", &[], Some("uint256"));
            scope.define_function(builder, "gasprice", &[], Some("uint256"));
            scope.define_function(builder, "gt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "invalid", &[], None);
            scope.define_function(builder, "iszero", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "jump", &[], None);
            scope.define_function(builder, "jumpi", &[], None);
            scope.define_function(builder, "log0", &["uint256 p", "uint256 s"], None);
            scope.define_function(
                builder,
                "log1",
                &["uint256 p", "uint256 s", "uint256 t1"],
                None,
            );
            scope.define_function(
                builder,
                "log2",
                &["uint256 p", "uint256 s", "uint256 t1", "uint256 t2"],
                None,
            );
            scope.define_function(
                builder,
                "log3",
                &[
                    "uint256 p",
                    "uint256 s",
                    "uint256 t1",
                    "uint256 t2",
                    "uint256 t3",
                ],
                None,
            );
            scope.define_function(
                builder,
                "log4",
                &[
                    "uint256 p",
                    "uint256 s",
                    "uint256 t1",
                    "uint256 t2",
                    "uint256 t3",
                ],
                None,
            );
            scope.define_function(builder, "lt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "mload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "mod", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "msize", &[], Some("uint256"));
            scope.define_function(builder, "mstore8", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "mstore", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "mul", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "mulmod",
                &["uint256 x", "uint256 y", "uint256 m"],
                Some("uint256"),
            );
            scope.define_function(builder, "not", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "number", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "origin", &[], Some("uint256"));
            scope.define_function(builder, "or", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "pop", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "return", &["uint256 p", "uint256 s"], None);
            scope.define_function(builder, "revert", &["uint256 p", "uint256 s"], None);
            scope.define_function(
                builder,
                "sdiv",
                &["uint256 x", "uint256 y"],
                Some("uint256"),
            );
            scope.define_function(builder, "selfdestruct", &["uint256 a"], None);
            scope.define_function(builder, "sgt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "signextend",
                &["uint256 i", "uint256 x"],
                Some("uint256"),
            );
            scope.define_function(builder, "sload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "slt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "smod",
                &["uint256 x", "uint256 y"],
                Some("uint256"),
            );
            scope.define_function(builder, "sstore", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "stop", &[], None);
            scope.define_function(builder, "sub", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "timestamp", &[], Some("uint256"));
            scope.define_function(builder, "xor", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "sha3", &[], Some("uint256"));
            scope.define_function(builder, "suicide", &[], Some("uint256"));
            scope.define_function(
                builder,
                "returndatacopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "returndatasize", &[], Some("uint256"));
            scope.define_function(builder, "sar", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "shl", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "shr", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "chainid", &[], Some("uint256"));
            scope.define_function(builder, "selfbalance", &[], Some("uint256"));
            scope.define_function(builder, "difficulty", &[], Some("uint256"));
        }
    } else if *version < Version::new(0, 4, 17) {
        // 0.4.12
        // SolidityBuiltIns
        {
            scope.define_function(
                builder,
                "addmod",
                &["uint x", "uint y", "uint k"],
                Some("uint256"),
            );
            scope.define_function(builder, "assert", &["bool condition"], None);
            scope.define_function(
                builder,
                "ecrecover",
                &["bytes32 hash", "uint8 v", "bytes32 r", "bytes32 s"],
                Some("address"),
            );
            scope.define_function(builder, "keccak256", &["bytes memory"], Some("bytes32"));
            scope.define_function(builder, "log0", &["bytes32"], None);
            scope.define_function(builder, "log1", &["bytes32", "bytes32"], None);
            scope.define_function(builder, "log2", &["bytes32", "bytes32", "bytes32"], None);
            scope.define_function(
                builder,
                "log3",
                &["bytes32", "bytes32", "bytes32", "bytes32"],
                None,
            );
            scope.define_function(
                builder,
                "log4",
                &["bytes32", "bytes32", "bytes32", "bytes32", "bytes32"],
                None,
            );
            scope.define_function(
                builder,
                "mulmod",
                &["uint x", "uint y", "uint k"],
                Some("uint256"),
            );
            scope.define_function(builder, "require", &["bool condition"], None);
            scope.define_function(builder, "revert", &[], None);
            scope.define_function(builder, "ripemd160", &["bytes memory"], Some("bytes20"));
            scope.define_function(
                builder,
                "selfdestruct",
                &["address payable recipient"],
                None,
            );
            scope.define_function(builder, "sha256", &["bytes memory"], Some("bytes32"));
            scope.define_function(builder, "sha3", &["bytes memory"], Some("bytes32"));
            scope.define_function(builder, "suicide", &["address payable recipient"], None);
            _ = scope.define_type(builder, "%AbiType");
            let mut type_scope = scope.define_type(builder, "address");
            type_scope.define_field(builder, "balance", "uint256");
            type_scope.define_function(builder, "call", &["bytes memory"], Some("bool"));
            type_scope.define_function(
                builder,
                "callcode",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "delegatecall", &["bytes memory"], Some("bool"));
            type_scope.define_function(builder, "send", &["uint256 amount"], Some("bool"));
            type_scope.define_function(builder, "transfer", &["uint256 amount"], None);
            let mut type_scope = scope.define_type(builder, "%Array");
            type_scope.define_field(builder, "length", "uint256");
            type_scope.define_function(builder, "push", &["%ValueType element"], Some("uint256"));
            type_scope.define_function(builder, "pop", &[], None);
            let mut type_scope = scope.define_type(builder, "%FixedArray");
            type_scope.define_field(builder, "length", "uint256");
            let mut type_scope = scope.define_type(builder, "%BlockType");
            type_scope.define_field(builder, "coinbase", "address payable");
            type_scope.define_field(builder, "difficulty", "uint256");
            type_scope.define_field(builder, "gaslimit", "uint256");
            type_scope.define_field(builder, "number", "uint256");
            type_scope.define_field(builder, "timestamp", "uint256");
            type_scope.define_function(builder, "blockhash", &["uint"], Some("bytes32"));
            let mut type_scope = scope.define_type(builder, "bytes");
            type_scope.define_field(builder, "length", "uint256");
            let mut type_scope = scope.define_type(builder, "%BytesType");
            type_scope.define_function(
                builder,
                "concat",
                &["bytes[] bytesToConcatenate"],
                Some("bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "%Function");
            type_scope.define_function(builder, "gas", &["uint amount"], Some("function()"));
            type_scope.define_function(builder, "value", &["uint amount"], Some("function()"));
            let mut type_scope = scope.define_type(builder, "%ExternalFunction");
            type_scope.define_function(builder, "gas", &["uint amount"], Some("function()"));
            type_scope.define_function(builder, "value", &["uint amount"], Some("function()"));
            let mut type_scope = scope.define_type(builder, "%MessageType");
            type_scope.define_field(builder, "data", "bytes");
            type_scope.define_field(builder, "gas", "uint256");
            type_scope.define_field(builder, "sender", "address payable");
            type_scope.define_field(builder, "sig", "bytes4");
            type_scope.define_field(builder, "value", "uint256");
            let mut type_scope = scope.define_type(builder, "%StringType");
            type_scope.define_function(
                builder,
                "concat",
                &["string[] stringsToConcatenate"],
                Some("string memory"),
            );
            let mut type_scope = scope.define_type(builder, "%TransactionType");
            type_scope.define_field(builder, "gasprice", "uint256");
            type_scope.define_field(builder, "origin", "address payable");
            let mut type_scope = scope.define_type(builder, "%ContractTypeType");
            type_scope.define_field(builder, "name", "string");
            let mut type_scope = scope.define_type(builder, "%InterfaceTypeType");
            type_scope.define_field(builder, "name", "string");
            _ = scope.define_type(builder, "%IntTypeType");
            scope.define_field(builder, "%_", "%Function");
            scope.define_field(builder, "abi", "%AbiType");
            scope.define_field(builder, "block", "%BlockType");
            scope.define_field(builder, "bytes", "%BytesType");
            scope.define_field(builder, "msg", "%MessageType");
            scope.define_field(builder, "now", "uint256");
            scope.define_field(builder, "string", "%StringType");
            scope.define_field(builder, "tx", "%TransactionType");
        }
        // YulBuiltIns
        {
            let mut scope = scope.new_context(builder, "@yul");
            let mut type_scope = scope.define_type(builder, "%YulExternal");
            type_scope.define_field(builder, "slot", "uint256");
            type_scope.define_field(builder, "offset", "uint256");
            type_scope.define_field(builder, "length", "uint256");
            scope.define_function(builder, "add", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "addmod",
                &["uint256 x", "uint256 y", "uint256 m"],
                Some("uint256"),
            );
            scope.define_function(builder, "address", &[], Some("uint256"));
            scope.define_function(builder, "and", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "balance", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "blockhash", &["uint256 b"], Some("uint256"));
            scope.define_function(
                builder,
                "byte",
                &["uint256 n", "uint256 x"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "callcode",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 v",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "calldatacopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "calldataload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "calldatasize", &[], Some("uint256"));
            scope.define_function(builder, "caller", &[], Some("uint256"));
            scope.define_function(
                builder,
                "call",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 v",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(builder, "callvalue", &[], Some("uint256"));
            scope.define_function(
                builder,
                "codecopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "codesize", &[], Some("uint256"));
            scope.define_function(builder, "coinbase", &[], Some("uint256"));
            scope.define_function(
                builder,
                "create",
                &["uint256 v", "uint256 p", "uint256 n"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "delegatecall",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(builder, "div", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "eq", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "exp", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "extcodecopy",
                &["uint256 a", "uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "extcodesize", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "gas", &[], Some("uint256"));
            scope.define_function(builder, "gaslimit", &[], Some("uint256"));
            scope.define_function(builder, "gasprice", &[], Some("uint256"));
            scope.define_function(builder, "gt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "invalid", &[], None);
            scope.define_function(builder, "iszero", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "jump", &[], None);
            scope.define_function(builder, "jumpi", &[], None);
            scope.define_function(builder, "log0", &["uint256 p", "uint256 s"], None);
            scope.define_function(
                builder,
                "log1",
                &["uint256 p", "uint256 s", "uint256 t1"],
                None,
            );
            scope.define_function(
                builder,
                "log2",
                &["uint256 p", "uint256 s", "uint256 t1", "uint256 t2"],
                None,
            );
            scope.define_function(
                builder,
                "log3",
                &[
                    "uint256 p",
                    "uint256 s",
                    "uint256 t1",
                    "uint256 t2",
                    "uint256 t3",
                ],
                None,
            );
            scope.define_function(
                builder,
                "log4",
                &[
                    "uint256 p",
                    "uint256 s",
                    "uint256 t1",
                    "uint256 t2",
                    "uint256 t3",
                ],
                None,
            );
            scope.define_function(builder, "lt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "mload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "mod", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "msize", &[], Some("uint256"));
            scope.define_function(builder, "mstore8", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "mstore", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "mul", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "mulmod",
                &["uint256 x", "uint256 y", "uint256 m"],
                Some("uint256"),
            );
            scope.define_function(builder, "not", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "number", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "origin", &[], Some("uint256"));
            scope.define_function(builder, "or", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "pop", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "return", &["uint256 p", "uint256 s"], None);
            scope.define_function(builder, "revert", &["uint256 p", "uint256 s"], None);
            scope.define_function(
                builder,
                "sdiv",
                &["uint256 x", "uint256 y"],
                Some("uint256"),
            );
            scope.define_function(builder, "selfdestruct", &["uint256 a"], None);
            scope.define_function(builder, "sgt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "signextend",
                &["uint256 i", "uint256 x"],
                Some("uint256"),
            );
            scope.define_function(builder, "sload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "slt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "smod",
                &["uint256 x", "uint256 y"],
                Some("uint256"),
            );
            scope.define_function(builder, "sstore", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "stop", &[], None);
            scope.define_function(builder, "sub", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "timestamp", &[], Some("uint256"));
            scope.define_function(builder, "xor", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "keccak256",
                &["uint256 p", "uint256 n"],
                Some("uint256"),
            );
            scope.define_function(builder, "sha3", &[], Some("uint256"));
            scope.define_function(builder, "suicide", &[], Some("uint256"));
            scope.define_function(
                builder,
                "returndatacopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "returndatasize", &[], Some("uint256"));
            scope.define_function(
                builder,
                "staticcall",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "create2",
                &["uint256 v", "uint256 p", "uint256 n", "uint256 s"],
                Some("uint256"),
            );
            scope.define_function(builder, "sar", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "shl", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "shr", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "chainid", &[], Some("uint256"));
            scope.define_function(builder, "selfbalance", &[], Some("uint256"));
            scope.define_function(builder, "difficulty", &[], Some("uint256"));
        }
    } else if *version < Version::new(0, 4, 22) {
        // 0.4.17
        // SolidityBuiltIns
        {
            scope.define_function(
                builder,
                "addmod",
                &["uint x", "uint y", "uint k"],
                Some("uint256"),
            );
            scope.define_function(builder, "assert", &["bool condition"], None);
            scope.define_function(
                builder,
                "ecrecover",
                &["bytes32 hash", "uint8 v", "bytes32 r", "bytes32 s"],
                Some("address"),
            );
            scope.define_function(builder, "keccak256", &["bytes memory"], Some("bytes32"));
            scope.define_function(builder, "log0", &["bytes32"], None);
            scope.define_function(builder, "log1", &["bytes32", "bytes32"], None);
            scope.define_function(builder, "log2", &["bytes32", "bytes32", "bytes32"], None);
            scope.define_function(
                builder,
                "log3",
                &["bytes32", "bytes32", "bytes32", "bytes32"],
                None,
            );
            scope.define_function(
                builder,
                "log4",
                &["bytes32", "bytes32", "bytes32", "bytes32", "bytes32"],
                None,
            );
            scope.define_function(
                builder,
                "mulmod",
                &["uint x", "uint y", "uint k"],
                Some("uint256"),
            );
            scope.define_function(builder, "require", &["bool condition"], None);
            scope.define_function(builder, "revert", &[], None);
            scope.define_function(builder, "ripemd160", &["bytes memory"], Some("bytes20"));
            scope.define_function(
                builder,
                "selfdestruct",
                &["address payable recipient"],
                None,
            );
            scope.define_function(builder, "sha256", &["bytes memory"], Some("bytes32"));
            scope.define_function(builder, "sha3", &["bytes memory"], Some("bytes32"));
            scope.define_function(builder, "suicide", &["address payable recipient"], None);
            _ = scope.define_type(builder, "%AbiType");
            let mut type_scope = scope.define_type(builder, "address");
            type_scope.define_field(builder, "balance", "uint256");
            type_scope.define_function(builder, "call", &["bytes memory"], Some("bool"));
            type_scope.define_function(
                builder,
                "callcode",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "delegatecall", &["bytes memory"], Some("bool"));
            type_scope.define_function(builder, "send", &["uint256 amount"], Some("bool"));
            type_scope.define_function(builder, "transfer", &["uint256 amount"], None);
            let mut type_scope = scope.define_type(builder, "%Array");
            type_scope.define_field(builder, "length", "uint256");
            type_scope.define_function(builder, "push", &["%ValueType element"], Some("uint256"));
            type_scope.define_function(builder, "pop", &[], None);
            let mut type_scope = scope.define_type(builder, "%FixedArray");
            type_scope.define_field(builder, "length", "uint256");
            let mut type_scope = scope.define_type(builder, "%BlockType");
            type_scope.define_field(builder, "coinbase", "address payable");
            type_scope.define_field(builder, "difficulty", "uint256");
            type_scope.define_field(builder, "gaslimit", "uint256");
            type_scope.define_field(builder, "number", "uint256");
            type_scope.define_field(builder, "timestamp", "uint256");
            type_scope.define_function(builder, "blockhash", &["uint"], Some("bytes32"));
            let mut type_scope = scope.define_type(builder, "bytes");
            type_scope.define_field(builder, "length", "uint256");
            let mut type_scope = scope.define_type(builder, "%BytesType");
            type_scope.define_function(
                builder,
                "concat",
                &["bytes[] bytesToConcatenate"],
                Some("bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "%Function");
            type_scope.define_function(builder, "gas", &["uint amount"], Some("function()"));
            type_scope.define_function(builder, "value", &["uint amount"], Some("function()"));
            let mut type_scope = scope.define_type(builder, "%ExternalFunction");
            type_scope.define_field(builder, "selector", "bytes4");
            type_scope.define_function(builder, "gas", &["uint amount"], Some("function()"));
            type_scope.define_function(builder, "value", &["uint amount"], Some("function()"));
            let mut type_scope = scope.define_type(builder, "%MessageType");
            type_scope.define_field(builder, "data", "bytes");
            type_scope.define_field(builder, "gas", "uint256");
            type_scope.define_field(builder, "sender", "address payable");
            type_scope.define_field(builder, "sig", "bytes4");
            type_scope.define_field(builder, "value", "uint256");
            let mut type_scope = scope.define_type(builder, "%StringType");
            type_scope.define_function(
                builder,
                "concat",
                &["string[] stringsToConcatenate"],
                Some("string memory"),
            );
            let mut type_scope = scope.define_type(builder, "%TransactionType");
            type_scope.define_field(builder, "gasprice", "uint256");
            type_scope.define_field(builder, "origin", "address payable");
            let mut type_scope = scope.define_type(builder, "%ContractTypeType");
            type_scope.define_field(builder, "name", "string");
            let mut type_scope = scope.define_type(builder, "%InterfaceTypeType");
            type_scope.define_field(builder, "name", "string");
            _ = scope.define_type(builder, "%IntTypeType");
            scope.define_field(builder, "%_", "%Function");
            scope.define_field(builder, "abi", "%AbiType");
            scope.define_field(builder, "block", "%BlockType");
            scope.define_field(builder, "bytes", "%BytesType");
            scope.define_field(builder, "msg", "%MessageType");
            scope.define_field(builder, "now", "uint256");
            scope.define_field(builder, "string", "%StringType");
            scope.define_field(builder, "tx", "%TransactionType");
        }
        // YulBuiltIns
        {
            let mut scope = scope.new_context(builder, "@yul");
            let mut type_scope = scope.define_type(builder, "%YulExternal");
            type_scope.define_field(builder, "slot", "uint256");
            type_scope.define_field(builder, "offset", "uint256");
            type_scope.define_field(builder, "length", "uint256");
            scope.define_function(builder, "add", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "addmod",
                &["uint256 x", "uint256 y", "uint256 m"],
                Some("uint256"),
            );
            scope.define_function(builder, "address", &[], Some("uint256"));
            scope.define_function(builder, "and", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "balance", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "blockhash", &["uint256 b"], Some("uint256"));
            scope.define_function(
                builder,
                "byte",
                &["uint256 n", "uint256 x"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "callcode",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 v",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "calldatacopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "calldataload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "calldatasize", &[], Some("uint256"));
            scope.define_function(builder, "caller", &[], Some("uint256"));
            scope.define_function(
                builder,
                "call",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 v",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(builder, "callvalue", &[], Some("uint256"));
            scope.define_function(
                builder,
                "codecopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "codesize", &[], Some("uint256"));
            scope.define_function(builder, "coinbase", &[], Some("uint256"));
            scope.define_function(
                builder,
                "create",
                &["uint256 v", "uint256 p", "uint256 n"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "delegatecall",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(builder, "div", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "eq", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "exp", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "extcodecopy",
                &["uint256 a", "uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "extcodesize", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "gas", &[], Some("uint256"));
            scope.define_function(builder, "gaslimit", &[], Some("uint256"));
            scope.define_function(builder, "gasprice", &[], Some("uint256"));
            scope.define_function(builder, "gt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "invalid", &[], None);
            scope.define_function(builder, "iszero", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "jump", &[], None);
            scope.define_function(builder, "jumpi", &[], None);
            scope.define_function(builder, "log0", &["uint256 p", "uint256 s"], None);
            scope.define_function(
                builder,
                "log1",
                &["uint256 p", "uint256 s", "uint256 t1"],
                None,
            );
            scope.define_function(
                builder,
                "log2",
                &["uint256 p", "uint256 s", "uint256 t1", "uint256 t2"],
                None,
            );
            scope.define_function(
                builder,
                "log3",
                &[
                    "uint256 p",
                    "uint256 s",
                    "uint256 t1",
                    "uint256 t2",
                    "uint256 t3",
                ],
                None,
            );
            scope.define_function(
                builder,
                "log4",
                &[
                    "uint256 p",
                    "uint256 s",
                    "uint256 t1",
                    "uint256 t2",
                    "uint256 t3",
                ],
                None,
            );
            scope.define_function(builder, "lt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "mload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "mod", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "msize", &[], Some("uint256"));
            scope.define_function(builder, "mstore8", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "mstore", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "mul", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "mulmod",
                &["uint256 x", "uint256 y", "uint256 m"],
                Some("uint256"),
            );
            scope.define_function(builder, "not", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "number", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "origin", &[], Some("uint256"));
            scope.define_function(builder, "or", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "pop", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "return", &["uint256 p", "uint256 s"], None);
            scope.define_function(builder, "revert", &["uint256 p", "uint256 s"], None);
            scope.define_function(
                builder,
                "sdiv",
                &["uint256 x", "uint256 y"],
                Some("uint256"),
            );
            scope.define_function(builder, "selfdestruct", &["uint256 a"], None);
            scope.define_function(builder, "sgt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "signextend",
                &["uint256 i", "uint256 x"],
                Some("uint256"),
            );
            scope.define_function(builder, "sload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "slt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "smod",
                &["uint256 x", "uint256 y"],
                Some("uint256"),
            );
            scope.define_function(builder, "sstore", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "stop", &[], None);
            scope.define_function(builder, "sub", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "timestamp", &[], Some("uint256"));
            scope.define_function(builder, "xor", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "keccak256",
                &["uint256 p", "uint256 n"],
                Some("uint256"),
            );
            scope.define_function(builder, "sha3", &[], Some("uint256"));
            scope.define_function(builder, "suicide", &[], Some("uint256"));
            scope.define_function(
                builder,
                "returndatacopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "returndatasize", &[], Some("uint256"));
            scope.define_function(
                builder,
                "staticcall",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "create2",
                &["uint256 v", "uint256 p", "uint256 n", "uint256 s"],
                Some("uint256"),
            );
            scope.define_function(builder, "sar", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "shl", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "shr", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "chainid", &[], Some("uint256"));
            scope.define_function(builder, "selfbalance", &[], Some("uint256"));
            scope.define_function(builder, "difficulty", &[], Some("uint256"));
        }
    } else if *version < Version::new(0, 5, 0) {
        // 0.4.22
        // SolidityBuiltIns
        {
            scope.define_function(
                builder,
                "addmod",
                &["uint x", "uint y", "uint k"],
                Some("uint256"),
            );
            scope.define_function(builder, "assert", &["bool condition"], None);
            scope.define_function(builder, "blockhash", &["uint blockNumber"], Some("bytes32"));
            scope.define_function(
                builder,
                "ecrecover",
                &["bytes32 hash", "uint8 v", "bytes32 r", "bytes32 s"],
                Some("address"),
            );
            scope.define_function(builder, "gasleft", &[], Some("uint256"));
            scope.define_function(builder, "keccak256", &["bytes memory"], Some("bytes32"));
            scope.define_function(builder, "log0", &["bytes32"], None);
            scope.define_function(builder, "log1", &["bytes32", "bytes32"], None);
            scope.define_function(builder, "log2", &["bytes32", "bytes32", "bytes32"], None);
            scope.define_function(
                builder,
                "log3",
                &["bytes32", "bytes32", "bytes32", "bytes32"],
                None,
            );
            scope.define_function(
                builder,
                "log4",
                &["bytes32", "bytes32", "bytes32", "bytes32", "bytes32"],
                None,
            );
            scope.define_function(
                builder,
                "mulmod",
                &["uint x", "uint y", "uint k"],
                Some("uint256"),
            );
            scope.define_function(builder, "require", &["bool condition"], None);
            scope.define_function(
                builder,
                "require",
                &["bool condition", "string memory message"],
                None,
            );
            scope.define_function(builder, "revert", &[], None);
            scope.define_function(builder, "revert", &["string memory reason"], None);
            scope.define_function(builder, "ripemd160", &["bytes memory"], Some("bytes20"));
            scope.define_function(
                builder,
                "selfdestruct",
                &["address payable recipient"],
                None,
            );
            scope.define_function(builder, "sha256", &["bytes memory"], Some("bytes32"));
            scope.define_function(builder, "sha3", &["bytes memory"], Some("bytes32"));
            scope.define_function(builder, "suicide", &["address payable recipient"], None);
            let mut type_scope = scope.define_type(builder, "%AbiType");
            type_scope.define_function(
                builder,
                "encode",
                &["%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodePacked",
                &["%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodeWithSelector",
                &["bytes4 selector", "%Any[] functionArgumentsTuple"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodeWithSignature",
                &["string memory signature", "%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "address");
            type_scope.define_field(builder, "balance", "uint256");
            type_scope.define_function(builder, "call", &["bytes memory"], Some("bool"));
            type_scope.define_function(
                builder,
                "callcode",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "delegatecall", &["bytes memory"], Some("bool"));
            type_scope.define_function(builder, "send", &["uint256 amount"], Some("bool"));
            type_scope.define_function(builder, "transfer", &["uint256 amount"], None);
            let mut type_scope = scope.define_type(builder, "%Array");
            type_scope.define_field(builder, "length", "uint256");
            type_scope.define_function(builder, "push", &["%ValueType element"], Some("uint256"));
            type_scope.define_function(builder, "pop", &[], None);
            let mut type_scope = scope.define_type(builder, "%FixedArray");
            type_scope.define_field(builder, "length", "uint256");
            let mut type_scope = scope.define_type(builder, "%BlockType");
            type_scope.define_field(builder, "coinbase", "address payable");
            type_scope.define_field(builder, "difficulty", "uint256");
            type_scope.define_field(builder, "gaslimit", "uint256");
            type_scope.define_field(builder, "number", "uint256");
            type_scope.define_field(builder, "timestamp", "uint256");
            type_scope.define_function(builder, "blockhash", &["uint"], Some("bytes32"));
            let mut type_scope = scope.define_type(builder, "bytes");
            type_scope.define_field(builder, "length", "uint256");
            let mut type_scope = scope.define_type(builder, "%BytesType");
            type_scope.define_function(
                builder,
                "concat",
                &["bytes[] bytesToConcatenate"],
                Some("bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "%Function");
            type_scope.define_function(builder, "gas", &["uint amount"], Some("function()"));
            type_scope.define_function(builder, "value", &["uint amount"], Some("function()"));
            let mut type_scope = scope.define_type(builder, "%ExternalFunction");
            type_scope.define_field(builder, "selector", "bytes4");
            type_scope.define_function(builder, "gas", &["uint amount"], Some("function()"));
            type_scope.define_function(builder, "value", &["uint amount"], Some("function()"));
            let mut type_scope = scope.define_type(builder, "%MessageType");
            type_scope.define_field(builder, "data", "bytes");
            type_scope.define_field(builder, "gas", "uint256");
            type_scope.define_field(builder, "sender", "address payable");
            type_scope.define_field(builder, "sig", "bytes4");
            type_scope.define_field(builder, "value", "uint256");
            let mut type_scope = scope.define_type(builder, "%StringType");
            type_scope.define_function(
                builder,
                "concat",
                &["string[] stringsToConcatenate"],
                Some("string memory"),
            );
            let mut type_scope = scope.define_type(builder, "%TransactionType");
            type_scope.define_field(builder, "gasprice", "uint256");
            type_scope.define_field(builder, "origin", "address payable");
            let mut type_scope = scope.define_type(builder, "%ContractTypeType");
            type_scope.define_field(builder, "name", "string");
            let mut type_scope = scope.define_type(builder, "%InterfaceTypeType");
            type_scope.define_field(builder, "name", "string");
            _ = scope.define_type(builder, "%IntTypeType");
            scope.define_field(builder, "%_", "%Function");
            scope.define_field(builder, "abi", "%AbiType");
            scope.define_field(builder, "block", "%BlockType");
            scope.define_field(builder, "bytes", "%BytesType");
            scope.define_field(builder, "msg", "%MessageType");
            scope.define_field(builder, "now", "uint256");
            scope.define_field(builder, "string", "%StringType");
            scope.define_field(builder, "tx", "%TransactionType");
        }
        // YulBuiltIns
        {
            let mut scope = scope.new_context(builder, "@yul");
            let mut type_scope = scope.define_type(builder, "%YulExternal");
            type_scope.define_field(builder, "slot", "uint256");
            type_scope.define_field(builder, "offset", "uint256");
            type_scope.define_field(builder, "length", "uint256");
            scope.define_function(builder, "add", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "addmod",
                &["uint256 x", "uint256 y", "uint256 m"],
                Some("uint256"),
            );
            scope.define_function(builder, "address", &[], Some("uint256"));
            scope.define_function(builder, "and", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "balance", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "blockhash", &["uint256 b"], Some("uint256"));
            scope.define_function(
                builder,
                "byte",
                &["uint256 n", "uint256 x"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "callcode",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 v",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "calldatacopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "calldataload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "calldatasize", &[], Some("uint256"));
            scope.define_function(builder, "caller", &[], Some("uint256"));
            scope.define_function(
                builder,
                "call",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 v",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(builder, "callvalue", &[], Some("uint256"));
            scope.define_function(
                builder,
                "codecopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "codesize", &[], Some("uint256"));
            scope.define_function(builder, "coinbase", &[], Some("uint256"));
            scope.define_function(
                builder,
                "create",
                &["uint256 v", "uint256 p", "uint256 n"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "delegatecall",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(builder, "div", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "eq", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "exp", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "extcodecopy",
                &["uint256 a", "uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "extcodesize", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "gas", &[], Some("uint256"));
            scope.define_function(builder, "gaslimit", &[], Some("uint256"));
            scope.define_function(builder, "gasprice", &[], Some("uint256"));
            scope.define_function(builder, "gt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "invalid", &[], None);
            scope.define_function(builder, "iszero", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "jump", &[], None);
            scope.define_function(builder, "jumpi", &[], None);
            scope.define_function(builder, "log0", &["uint256 p", "uint256 s"], None);
            scope.define_function(
                builder,
                "log1",
                &["uint256 p", "uint256 s", "uint256 t1"],
                None,
            );
            scope.define_function(
                builder,
                "log2",
                &["uint256 p", "uint256 s", "uint256 t1", "uint256 t2"],
                None,
            );
            scope.define_function(
                builder,
                "log3",
                &[
                    "uint256 p",
                    "uint256 s",
                    "uint256 t1",
                    "uint256 t2",
                    "uint256 t3",
                ],
                None,
            );
            scope.define_function(
                builder,
                "log4",
                &[
                    "uint256 p",
                    "uint256 s",
                    "uint256 t1",
                    "uint256 t2",
                    "uint256 t3",
                ],
                None,
            );
            scope.define_function(builder, "lt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "mload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "mod", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "msize", &[], Some("uint256"));
            scope.define_function(builder, "mstore8", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "mstore", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "mul", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "mulmod",
                &["uint256 x", "uint256 y", "uint256 m"],
                Some("uint256"),
            );
            scope.define_function(builder, "not", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "number", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "origin", &[], Some("uint256"));
            scope.define_function(builder, "or", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "pop", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "return", &["uint256 p", "uint256 s"], None);
            scope.define_function(builder, "revert", &["uint256 p", "uint256 s"], None);
            scope.define_function(
                builder,
                "sdiv",
                &["uint256 x", "uint256 y"],
                Some("uint256"),
            );
            scope.define_function(builder, "selfdestruct", &["uint256 a"], None);
            scope.define_function(builder, "sgt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "signextend",
                &["uint256 i", "uint256 x"],
                Some("uint256"),
            );
            scope.define_function(builder, "sload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "slt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "smod",
                &["uint256 x", "uint256 y"],
                Some("uint256"),
            );
            scope.define_function(builder, "sstore", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "stop", &[], None);
            scope.define_function(builder, "sub", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "timestamp", &[], Some("uint256"));
            scope.define_function(builder, "xor", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "keccak256",
                &["uint256 p", "uint256 n"],
                Some("uint256"),
            );
            scope.define_function(builder, "sha3", &[], Some("uint256"));
            scope.define_function(builder, "suicide", &[], Some("uint256"));
            scope.define_function(
                builder,
                "returndatacopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "returndatasize", &[], Some("uint256"));
            scope.define_function(
                builder,
                "staticcall",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "create2",
                &["uint256 v", "uint256 p", "uint256 n", "uint256 s"],
                Some("uint256"),
            );
            scope.define_function(builder, "sar", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "shl", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "shr", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "chainid", &[], Some("uint256"));
            scope.define_function(builder, "selfbalance", &[], Some("uint256"));
            scope.define_function(builder, "difficulty", &[], Some("uint256"));
        }
    } else if *version < Version::new(0, 5, 3) {
        // 0.5.0
        // SolidityBuiltIns
        {
            scope.define_function(
                builder,
                "addmod",
                &["uint x", "uint y", "uint k"],
                Some("uint256"),
            );
            scope.define_function(builder, "assert", &["bool condition"], None);
            scope.define_function(builder, "blockhash", &["uint blockNumber"], Some("bytes32"));
            scope.define_function(
                builder,
                "ecrecover",
                &["bytes32 hash", "uint8 v", "bytes32 r", "bytes32 s"],
                Some("address"),
            );
            scope.define_function(builder, "gasleft", &[], Some("uint256"));
            scope.define_function(builder, "keccak256", &["bytes memory"], Some("bytes32"));
            scope.define_function(builder, "log0", &["bytes32"], None);
            scope.define_function(builder, "log1", &["bytes32", "bytes32"], None);
            scope.define_function(builder, "log2", &["bytes32", "bytes32", "bytes32"], None);
            scope.define_function(
                builder,
                "log3",
                &["bytes32", "bytes32", "bytes32", "bytes32"],
                None,
            );
            scope.define_function(
                builder,
                "log4",
                &["bytes32", "bytes32", "bytes32", "bytes32", "bytes32"],
                None,
            );
            scope.define_function(
                builder,
                "mulmod",
                &["uint x", "uint y", "uint k"],
                Some("uint256"),
            );
            scope.define_function(builder, "require", &["bool condition"], None);
            scope.define_function(
                builder,
                "require",
                &["bool condition", "string memory message"],
                None,
            );
            scope.define_function(builder, "revert", &[], None);
            scope.define_function(builder, "revert", &["string memory reason"], None);
            scope.define_function(builder, "ripemd160", &["bytes memory"], Some("bytes20"));
            scope.define_function(
                builder,
                "selfdestruct",
                &["address payable recipient"],
                None,
            );
            scope.define_function(builder, "sha256", &["bytes memory"], Some("bytes32"));
            let mut type_scope = scope.define_type(builder, "%AbiType");
            type_scope.define_function(
                builder,
                "decode",
                &["bytes memory encodedData", "%Type[] encodedTypesTuple"],
                Some("%Any[]"),
            );
            type_scope.define_function(
                builder,
                "encode",
                &["%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodePacked",
                &["%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodeWithSelector",
                &["bytes4 selector", "%Any[] functionArgumentsTuple"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodeWithSignature",
                &["string memory signature", "%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "address");
            type_scope.define_field(builder, "balance", "uint256");
            type_scope.define_function(
                builder,
                "call",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(
                builder,
                "delegatecall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "send", &["uint256 amount"], Some("bool"));
            type_scope.define_function(
                builder,
                "staticcall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "address payable");
            type_scope.define_field(builder, "balance", "uint256");
            type_scope.define_function(
                builder,
                "call",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(
                builder,
                "delegatecall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "send", &["uint256 amount"], Some("bool"));
            type_scope.define_function(
                builder,
                "staticcall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "transfer", &["uint256 amount"], None);
            let mut type_scope = scope.define_type(builder, "%Array");
            type_scope.define_field(builder, "length", "uint256");
            type_scope.define_function(builder, "push", &["%ValueType element"], Some("uint256"));
            type_scope.define_function(builder, "pop", &[], None);
            let mut type_scope = scope.define_type(builder, "%FixedArray");
            type_scope.define_field(builder, "length", "uint256");
            let mut type_scope = scope.define_type(builder, "%BlockType");
            type_scope.define_field(builder, "coinbase", "address payable");
            type_scope.define_field(builder, "difficulty", "uint256");
            type_scope.define_field(builder, "gaslimit", "uint256");
            type_scope.define_field(builder, "number", "uint256");
            type_scope.define_field(builder, "timestamp", "uint256");
            let mut type_scope = scope.define_type(builder, "bytes");
            type_scope.define_field(builder, "length", "uint256");
            let mut type_scope = scope.define_type(builder, "%BytesType");
            type_scope.define_function(
                builder,
                "concat",
                &["bytes[] bytesToConcatenate"],
                Some("bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "%Function");
            type_scope.define_function(builder, "gas", &["uint amount"], Some("function()"));
            type_scope.define_function(builder, "value", &["uint amount"], Some("function()"));
            let mut type_scope = scope.define_type(builder, "%ExternalFunction");
            type_scope.define_field(builder, "selector", "bytes4");
            type_scope.define_function(builder, "gas", &["uint amount"], Some("function()"));
            type_scope.define_function(builder, "value", &["uint amount"], Some("function()"));
            let mut type_scope = scope.define_type(builder, "%MessageType");
            type_scope.define_field(builder, "data", "bytes");
            type_scope.define_field(builder, "sender", "address payable");
            type_scope.define_field(builder, "sig", "bytes4");
            type_scope.define_field(builder, "value", "uint256");
            let mut type_scope = scope.define_type(builder, "%StringType");
            type_scope.define_function(
                builder,
                "concat",
                &["string[] stringsToConcatenate"],
                Some("string memory"),
            );
            let mut type_scope = scope.define_type(builder, "%TransactionType");
            type_scope.define_field(builder, "gasprice", "uint256");
            type_scope.define_field(builder, "origin", "address payable");
            let mut type_scope = scope.define_type(builder, "%ContractTypeType");
            type_scope.define_field(builder, "name", "string");
            let mut type_scope = scope.define_type(builder, "%InterfaceTypeType");
            type_scope.define_field(builder, "name", "string");
            _ = scope.define_type(builder, "%IntTypeType");
            scope.define_field(builder, "%_", "%Function");
            scope.define_field(builder, "abi", "%AbiType");
            scope.define_field(builder, "block", "%BlockType");
            scope.define_field(builder, "bytes", "%BytesType");
            scope.define_field(builder, "msg", "%MessageType");
            scope.define_field(builder, "now", "uint256");
            scope.define_field(builder, "string", "%StringType");
            scope.define_field(builder, "tx", "%TransactionType");
        }
        // YulBuiltIns
        {
            let mut scope = scope.new_context(builder, "@yul");
            let mut type_scope = scope.define_type(builder, "%YulExternal");
            type_scope.define_field(builder, "slot", "uint256");
            type_scope.define_field(builder, "offset", "uint256");
            type_scope.define_field(builder, "length", "uint256");
            scope.define_function(builder, "add", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "addmod",
                &["uint256 x", "uint256 y", "uint256 m"],
                Some("uint256"),
            );
            scope.define_function(builder, "address", &[], Some("uint256"));
            scope.define_function(builder, "and", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "balance", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "blockhash", &["uint256 b"], Some("uint256"));
            scope.define_function(
                builder,
                "byte",
                &["uint256 n", "uint256 x"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "callcode",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 v",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "calldatacopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "calldataload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "calldatasize", &[], Some("uint256"));
            scope.define_function(builder, "caller", &[], Some("uint256"));
            scope.define_function(
                builder,
                "call",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 v",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(builder, "callvalue", &[], Some("uint256"));
            scope.define_function(
                builder,
                "codecopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "codesize", &[], Some("uint256"));
            scope.define_function(builder, "coinbase", &[], Some("uint256"));
            scope.define_function(
                builder,
                "create",
                &["uint256 v", "uint256 p", "uint256 n"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "delegatecall",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(builder, "div", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "eq", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "exp", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "extcodecopy",
                &["uint256 a", "uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "extcodesize", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "gas", &[], Some("uint256"));
            scope.define_function(builder, "gaslimit", &[], Some("uint256"));
            scope.define_function(builder, "gasprice", &[], Some("uint256"));
            scope.define_function(builder, "gt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "invalid", &[], None);
            scope.define_function(builder, "iszero", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "log0", &["uint256 p", "uint256 s"], None);
            scope.define_function(
                builder,
                "log1",
                &["uint256 p", "uint256 s", "uint256 t1"],
                None,
            );
            scope.define_function(
                builder,
                "log2",
                &["uint256 p", "uint256 s", "uint256 t1", "uint256 t2"],
                None,
            );
            scope.define_function(
                builder,
                "log3",
                &[
                    "uint256 p",
                    "uint256 s",
                    "uint256 t1",
                    "uint256 t2",
                    "uint256 t3",
                ],
                None,
            );
            scope.define_function(
                builder,
                "log4",
                &[
                    "uint256 p",
                    "uint256 s",
                    "uint256 t1",
                    "uint256 t2",
                    "uint256 t3",
                ],
                None,
            );
            scope.define_function(builder, "lt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "mload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "mod", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "msize", &[], Some("uint256"));
            scope.define_function(builder, "mstore8", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "mstore", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "mul", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "mulmod",
                &["uint256 x", "uint256 y", "uint256 m"],
                Some("uint256"),
            );
            scope.define_function(builder, "not", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "number", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "origin", &[], Some("uint256"));
            scope.define_function(builder, "or", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "pop", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "return", &["uint256 p", "uint256 s"], None);
            scope.define_function(builder, "revert", &["uint256 p", "uint256 s"], None);
            scope.define_function(
                builder,
                "sdiv",
                &["uint256 x", "uint256 y"],
                Some("uint256"),
            );
            scope.define_function(builder, "selfdestruct", &["uint256 a"], None);
            scope.define_function(builder, "sgt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "signextend",
                &["uint256 i", "uint256 x"],
                Some("uint256"),
            );
            scope.define_function(builder, "sload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "slt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "smod",
                &["uint256 x", "uint256 y"],
                Some("uint256"),
            );
            scope.define_function(builder, "sstore", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "stop", &[], None);
            scope.define_function(builder, "sub", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "timestamp", &[], Some("uint256"));
            scope.define_function(builder, "xor", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "keccak256",
                &["uint256 p", "uint256 n"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "returndatacopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "returndatasize", &[], Some("uint256"));
            scope.define_function(
                builder,
                "staticcall",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "create2",
                &["uint256 v", "uint256 p", "uint256 n", "uint256 s"],
                Some("uint256"),
            );
            scope.define_function(builder, "extcodehash", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "sar", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "shl", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "shr", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "chainid", &[], Some("uint256"));
            scope.define_function(builder, "selfbalance", &[], Some("uint256"));
            scope.define_function(builder, "difficulty", &[], Some("uint256"));
        }
    } else if *version < Version::new(0, 6, 0) {
        // 0.5.3
        // SolidityBuiltIns
        {
            scope.define_function(
                builder,
                "addmod",
                &["uint x", "uint y", "uint k"],
                Some("uint256"),
            );
            scope.define_function(builder, "assert", &["bool condition"], None);
            scope.define_function(builder, "blockhash", &["uint blockNumber"], Some("bytes32"));
            scope.define_function(
                builder,
                "ecrecover",
                &["bytes32 hash", "uint8 v", "bytes32 r", "bytes32 s"],
                Some("address"),
            );
            scope.define_function(builder, "gasleft", &[], Some("uint256"));
            scope.define_function(builder, "keccak256", &["bytes memory"], Some("bytes32"));
            scope.define_function(builder, "log0", &["bytes32"], None);
            scope.define_function(builder, "log1", &["bytes32", "bytes32"], None);
            scope.define_function(builder, "log2", &["bytes32", "bytes32", "bytes32"], None);
            scope.define_function(
                builder,
                "log3",
                &["bytes32", "bytes32", "bytes32", "bytes32"],
                None,
            );
            scope.define_function(
                builder,
                "log4",
                &["bytes32", "bytes32", "bytes32", "bytes32", "bytes32"],
                None,
            );
            scope.define_function(
                builder,
                "mulmod",
                &["uint x", "uint y", "uint k"],
                Some("uint256"),
            );
            scope.define_function(builder, "require", &["bool condition"], None);
            scope.define_function(
                builder,
                "require",
                &["bool condition", "string memory message"],
                None,
            );
            scope.define_function(builder, "revert", &[], None);
            scope.define_function(builder, "revert", &["string memory reason"], None);
            scope.define_function(builder, "ripemd160", &["bytes memory"], Some("bytes20"));
            scope.define_function(
                builder,
                "selfdestruct",
                &["address payable recipient"],
                None,
            );
            scope.define_function(builder, "sha256", &["bytes memory"], Some("bytes32"));
            let mut type_scope = scope.define_type(builder, "%AbiType");
            type_scope.define_function(
                builder,
                "decode",
                &["bytes memory encodedData", "%Type[] encodedTypesTuple"],
                Some("%Any[]"),
            );
            type_scope.define_function(
                builder,
                "encode",
                &["%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodePacked",
                &["%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodeWithSelector",
                &["bytes4 selector", "%Any[] functionArgumentsTuple"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodeWithSignature",
                &["string memory signature", "%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "address");
            type_scope.define_field(builder, "balance", "uint256");
            type_scope.define_function(
                builder,
                "call",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(
                builder,
                "delegatecall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "send", &["uint256 amount"], Some("bool"));
            type_scope.define_function(
                builder,
                "staticcall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "address payable");
            type_scope.define_field(builder, "balance", "uint256");
            type_scope.define_function(
                builder,
                "call",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(
                builder,
                "delegatecall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "send", &["uint256 amount"], Some("bool"));
            type_scope.define_function(
                builder,
                "staticcall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "transfer", &["uint256 amount"], None);
            let mut type_scope = scope.define_type(builder, "%Array");
            type_scope.define_field(builder, "length", "uint256");
            type_scope.define_function(builder, "push", &["%ValueType element"], Some("uint256"));
            type_scope.define_function(builder, "pop", &[], None);
            let mut type_scope = scope.define_type(builder, "%FixedArray");
            type_scope.define_field(builder, "length", "uint256");
            let mut type_scope = scope.define_type(builder, "%BlockType");
            type_scope.define_field(builder, "coinbase", "address payable");
            type_scope.define_field(builder, "difficulty", "uint256");
            type_scope.define_field(builder, "gaslimit", "uint256");
            type_scope.define_field(builder, "number", "uint256");
            type_scope.define_field(builder, "timestamp", "uint256");
            let mut type_scope = scope.define_type(builder, "bytes");
            type_scope.define_field(builder, "length", "uint256");
            let mut type_scope = scope.define_type(builder, "%BytesType");
            type_scope.define_function(
                builder,
                "concat",
                &["bytes[] bytesToConcatenate"],
                Some("bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "%Function");
            type_scope.define_function(builder, "gas", &["uint amount"], Some("function()"));
            type_scope.define_function(builder, "value", &["uint amount"], Some("function()"));
            let mut type_scope = scope.define_type(builder, "%ExternalFunction");
            type_scope.define_field(builder, "selector", "bytes4");
            type_scope.define_function(builder, "gas", &["uint amount"], Some("function()"));
            type_scope.define_function(builder, "value", &["uint amount"], Some("function()"));
            let mut type_scope = scope.define_type(builder, "%MessageType");
            type_scope.define_field(builder, "data", "bytes");
            type_scope.define_field(builder, "sender", "address payable");
            type_scope.define_field(builder, "sig", "bytes4");
            type_scope.define_field(builder, "value", "uint256");
            let mut type_scope = scope.define_type(builder, "%StringType");
            type_scope.define_function(
                builder,
                "concat",
                &["string[] stringsToConcatenate"],
                Some("string memory"),
            );
            let mut type_scope = scope.define_type(builder, "%TransactionType");
            type_scope.define_field(builder, "gasprice", "uint256");
            type_scope.define_field(builder, "origin", "address payable");
            let mut type_scope = scope.define_type(builder, "%ContractTypeType");
            type_scope.define_field(builder, "name", "string");
            type_scope.define_field(builder, "creationCode", "bytes");
            type_scope.define_field(builder, "runtimeCode", "bytes");
            let mut type_scope = scope.define_type(builder, "%InterfaceTypeType");
            type_scope.define_field(builder, "name", "string");
            _ = scope.define_type(builder, "%IntTypeType");
            scope.define_field(builder, "%_", "%Function");
            scope.define_field(builder, "abi", "%AbiType");
            scope.define_field(builder, "block", "%BlockType");
            scope.define_field(builder, "bytes", "%BytesType");
            scope.define_field(builder, "msg", "%MessageType");
            scope.define_field(builder, "now", "uint256");
            scope.define_field(builder, "string", "%StringType");
            scope.define_field(builder, "tx", "%TransactionType");
        }
        // YulBuiltIns
        {
            let mut scope = scope.new_context(builder, "@yul");
            let mut type_scope = scope.define_type(builder, "%YulExternal");
            type_scope.define_field(builder, "slot", "uint256");
            type_scope.define_field(builder, "offset", "uint256");
            type_scope.define_field(builder, "length", "uint256");
            scope.define_function(builder, "add", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "addmod",
                &["uint256 x", "uint256 y", "uint256 m"],
                Some("uint256"),
            );
            scope.define_function(builder, "address", &[], Some("uint256"));
            scope.define_function(builder, "and", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "balance", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "blockhash", &["uint256 b"], Some("uint256"));
            scope.define_function(
                builder,
                "byte",
                &["uint256 n", "uint256 x"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "callcode",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 v",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "calldatacopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "calldataload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "calldatasize", &[], Some("uint256"));
            scope.define_function(builder, "caller", &[], Some("uint256"));
            scope.define_function(
                builder,
                "call",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 v",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(builder, "callvalue", &[], Some("uint256"));
            scope.define_function(
                builder,
                "codecopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "codesize", &[], Some("uint256"));
            scope.define_function(builder, "coinbase", &[], Some("uint256"));
            scope.define_function(
                builder,
                "create",
                &["uint256 v", "uint256 p", "uint256 n"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "delegatecall",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(builder, "div", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "eq", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "exp", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "extcodecopy",
                &["uint256 a", "uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "extcodesize", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "gas", &[], Some("uint256"));
            scope.define_function(builder, "gaslimit", &[], Some("uint256"));
            scope.define_function(builder, "gasprice", &[], Some("uint256"));
            scope.define_function(builder, "gt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "invalid", &[], None);
            scope.define_function(builder, "iszero", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "log0", &["uint256 p", "uint256 s"], None);
            scope.define_function(
                builder,
                "log1",
                &["uint256 p", "uint256 s", "uint256 t1"],
                None,
            );
            scope.define_function(
                builder,
                "log2",
                &["uint256 p", "uint256 s", "uint256 t1", "uint256 t2"],
                None,
            );
            scope.define_function(
                builder,
                "log3",
                &[
                    "uint256 p",
                    "uint256 s",
                    "uint256 t1",
                    "uint256 t2",
                    "uint256 t3",
                ],
                None,
            );
            scope.define_function(
                builder,
                "log4",
                &[
                    "uint256 p",
                    "uint256 s",
                    "uint256 t1",
                    "uint256 t2",
                    "uint256 t3",
                ],
                None,
            );
            scope.define_function(builder, "lt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "mload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "mod", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "msize", &[], Some("uint256"));
            scope.define_function(builder, "mstore8", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "mstore", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "mul", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "mulmod",
                &["uint256 x", "uint256 y", "uint256 m"],
                Some("uint256"),
            );
            scope.define_function(builder, "not", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "number", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "origin", &[], Some("uint256"));
            scope.define_function(builder, "or", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "pop", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "return", &["uint256 p", "uint256 s"], None);
            scope.define_function(builder, "revert", &["uint256 p", "uint256 s"], None);
            scope.define_function(
                builder,
                "sdiv",
                &["uint256 x", "uint256 y"],
                Some("uint256"),
            );
            scope.define_function(builder, "selfdestruct", &["uint256 a"], None);
            scope.define_function(builder, "sgt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "signextend",
                &["uint256 i", "uint256 x"],
                Some("uint256"),
            );
            scope.define_function(builder, "sload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "slt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "smod",
                &["uint256 x", "uint256 y"],
                Some("uint256"),
            );
            scope.define_function(builder, "sstore", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "stop", &[], None);
            scope.define_function(builder, "sub", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "timestamp", &[], Some("uint256"));
            scope.define_function(builder, "xor", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "keccak256",
                &["uint256 p", "uint256 n"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "returndatacopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "returndatasize", &[], Some("uint256"));
            scope.define_function(
                builder,
                "staticcall",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "create2",
                &["uint256 v", "uint256 p", "uint256 n", "uint256 s"],
                Some("uint256"),
            );
            scope.define_function(builder, "extcodehash", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "sar", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "shl", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "shr", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "chainid", &[], Some("uint256"));
            scope.define_function(builder, "selfbalance", &[], Some("uint256"));
            scope.define_function(builder, "difficulty", &[], Some("uint256"));
        }
    } else if *version < Version::new(0, 6, 2) {
        // 0.6.0
        // SolidityBuiltIns
        {
            scope.define_function(
                builder,
                "addmod",
                &["uint x", "uint y", "uint k"],
                Some("uint256"),
            );
            scope.define_function(builder, "assert", &["bool condition"], None);
            scope.define_function(builder, "blockhash", &["uint blockNumber"], Some("bytes32"));
            scope.define_function(
                builder,
                "ecrecover",
                &["bytes32 hash", "uint8 v", "bytes32 r", "bytes32 s"],
                Some("address"),
            );
            scope.define_function(builder, "gasleft", &[], Some("uint256"));
            scope.define_function(builder, "keccak256", &["bytes memory"], Some("bytes32"));
            scope.define_function(builder, "log0", &["bytes32"], None);
            scope.define_function(builder, "log1", &["bytes32", "bytes32"], None);
            scope.define_function(builder, "log2", &["bytes32", "bytes32", "bytes32"], None);
            scope.define_function(
                builder,
                "log3",
                &["bytes32", "bytes32", "bytes32", "bytes32"],
                None,
            );
            scope.define_function(
                builder,
                "log4",
                &["bytes32", "bytes32", "bytes32", "bytes32", "bytes32"],
                None,
            );
            scope.define_function(
                builder,
                "mulmod",
                &["uint x", "uint y", "uint k"],
                Some("uint256"),
            );
            scope.define_function(builder, "require", &["bool condition"], None);
            scope.define_function(
                builder,
                "require",
                &["bool condition", "string memory message"],
                None,
            );
            scope.define_function(builder, "revert", &[], None);
            scope.define_function(builder, "revert", &["string memory reason"], None);
            scope.define_function(builder, "ripemd160", &["bytes memory"], Some("bytes20"));
            scope.define_function(
                builder,
                "selfdestruct",
                &["address payable recipient"],
                None,
            );
            scope.define_function(builder, "sha256", &["bytes memory"], Some("bytes32"));
            let mut type_scope = scope.define_type(builder, "%AbiType");
            type_scope.define_function(
                builder,
                "decode",
                &["bytes memory encodedData", "%Type[] encodedTypesTuple"],
                Some("%Any[]"),
            );
            type_scope.define_function(
                builder,
                "encode",
                &["%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodePacked",
                &["%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodeWithSelector",
                &["bytes4 selector", "%Any[] functionArgumentsTuple"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodeWithSignature",
                &["string memory signature", "%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "address");
            type_scope.define_field(builder, "balance", "uint256");
            type_scope.define_function(
                builder,
                "call",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(
                builder,
                "delegatecall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "send", &["uint256 amount"], Some("bool"));
            type_scope.define_function(
                builder,
                "staticcall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "address payable");
            type_scope.define_field(builder, "balance", "uint256");
            type_scope.define_function(
                builder,
                "call",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(
                builder,
                "delegatecall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "send", &["uint256 amount"], Some("bool"));
            type_scope.define_function(
                builder,
                "staticcall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "transfer", &["uint256 amount"], None);
            let mut type_scope = scope.define_type(builder, "%Array");
            type_scope.define_field(builder, "length", "uint256");
            type_scope.define_function(builder, "push", &[], Some("%ValueType"));
            type_scope.define_function(builder, "push", &["%ValueType element"], None);
            type_scope.define_function(builder, "pop", &[], None);
            let mut type_scope = scope.define_type(builder, "%FixedArray");
            type_scope.define_field(builder, "length", "uint256");
            let mut type_scope = scope.define_type(builder, "%BlockType");
            type_scope.define_field(builder, "coinbase", "address payable");
            type_scope.define_field(builder, "difficulty", "uint256");
            type_scope.define_field(builder, "gaslimit", "uint256");
            type_scope.define_field(builder, "number", "uint256");
            type_scope.define_field(builder, "timestamp", "uint256");
            let mut type_scope = scope.define_type(builder, "bytes");
            type_scope.define_field(builder, "length", "uint256");
            let mut type_scope = scope.define_type(builder, "%BytesType");
            type_scope.define_function(
                builder,
                "concat",
                &["bytes[] bytesToConcatenate"],
                Some("bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "Error");
            type_scope.define_field(builder, "reason", "string");
            let mut type_scope = scope.define_type(builder, "%Function");
            type_scope.define_function(builder, "gas", &["uint amount"], Some("function()"));
            type_scope.define_function(builder, "value", &["uint amount"], Some("function()"));
            let mut type_scope = scope.define_type(builder, "%ExternalFunction");
            type_scope.define_field(builder, "selector", "bytes4");
            type_scope.define_function(builder, "gas", &["uint amount"], Some("function()"));
            type_scope.define_function(builder, "value", &["uint amount"], Some("function()"));
            let mut type_scope = scope.define_type(builder, "%MessageType");
            type_scope.define_field(builder, "data", "bytes");
            type_scope.define_field(builder, "sender", "address payable");
            type_scope.define_field(builder, "sig", "bytes4");
            type_scope.define_field(builder, "value", "uint256");
            let mut type_scope = scope.define_type(builder, "Panic");
            type_scope.define_field(builder, "errorCode", "uint256");
            let mut type_scope = scope.define_type(builder, "%StringType");
            type_scope.define_function(
                builder,
                "concat",
                &["string[] stringsToConcatenate"],
                Some("string memory"),
            );
            let mut type_scope = scope.define_type(builder, "%TransactionType");
            type_scope.define_field(builder, "gasprice", "uint256");
            type_scope.define_field(builder, "origin", "address payable");
            let mut type_scope = scope.define_type(builder, "%ContractTypeType");
            type_scope.define_field(builder, "name", "string");
            type_scope.define_field(builder, "creationCode", "bytes");
            type_scope.define_field(builder, "runtimeCode", "bytes");
            let mut type_scope = scope.define_type(builder, "%InterfaceTypeType");
            type_scope.define_field(builder, "name", "string");
            _ = scope.define_type(builder, "%IntTypeType");
            scope.define_field(builder, "%_", "%Function");
            scope.define_field(builder, "abi", "%AbiType");
            scope.define_field(builder, "block", "%BlockType");
            scope.define_field(builder, "bytes", "%BytesType");
            scope.define_field(builder, "msg", "%MessageType");
            scope.define_field(builder, "now", "uint256");
            scope.define_field(builder, "string", "%StringType");
            scope.define_field(builder, "tx", "%TransactionType");
        }
        // YulBuiltIns
        {
            let mut scope = scope.new_context(builder, "@yul");
            let mut type_scope = scope.define_type(builder, "%YulExternal");
            type_scope.define_field(builder, "slot", "uint256");
            type_scope.define_field(builder, "offset", "uint256");
            type_scope.define_field(builder, "length", "uint256");
            scope.define_function(builder, "add", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "addmod",
                &["uint256 x", "uint256 y", "uint256 m"],
                Some("uint256"),
            );
            scope.define_function(builder, "address", &[], Some("uint256"));
            scope.define_function(builder, "and", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "balance", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "blockhash", &["uint256 b"], Some("uint256"));
            scope.define_function(
                builder,
                "byte",
                &["uint256 n", "uint256 x"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "callcode",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 v",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "calldatacopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "calldataload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "calldatasize", &[], Some("uint256"));
            scope.define_function(builder, "caller", &[], Some("uint256"));
            scope.define_function(
                builder,
                "call",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 v",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(builder, "callvalue", &[], Some("uint256"));
            scope.define_function(
                builder,
                "codecopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "codesize", &[], Some("uint256"));
            scope.define_function(builder, "coinbase", &[], Some("uint256"));
            scope.define_function(
                builder,
                "create",
                &["uint256 v", "uint256 p", "uint256 n"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "delegatecall",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(builder, "div", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "eq", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "exp", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "extcodecopy",
                &["uint256 a", "uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "extcodesize", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "gas", &[], Some("uint256"));
            scope.define_function(builder, "gaslimit", &[], Some("uint256"));
            scope.define_function(builder, "gasprice", &[], Some("uint256"));
            scope.define_function(builder, "gt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "invalid", &[], None);
            scope.define_function(builder, "iszero", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "log0", &["uint256 p", "uint256 s"], None);
            scope.define_function(
                builder,
                "log1",
                &["uint256 p", "uint256 s", "uint256 t1"],
                None,
            );
            scope.define_function(
                builder,
                "log2",
                &["uint256 p", "uint256 s", "uint256 t1", "uint256 t2"],
                None,
            );
            scope.define_function(
                builder,
                "log3",
                &[
                    "uint256 p",
                    "uint256 s",
                    "uint256 t1",
                    "uint256 t2",
                    "uint256 t3",
                ],
                None,
            );
            scope.define_function(
                builder,
                "log4",
                &[
                    "uint256 p",
                    "uint256 s",
                    "uint256 t1",
                    "uint256 t2",
                    "uint256 t3",
                ],
                None,
            );
            scope.define_function(builder, "lt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "mload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "mod", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "msize", &[], Some("uint256"));
            scope.define_function(builder, "mstore8", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "mstore", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "mul", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "mulmod",
                &["uint256 x", "uint256 y", "uint256 m"],
                Some("uint256"),
            );
            scope.define_function(builder, "not", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "number", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "origin", &[], Some("uint256"));
            scope.define_function(builder, "or", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "pop", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "return", &["uint256 p", "uint256 s"], None);
            scope.define_function(builder, "revert", &["uint256 p", "uint256 s"], None);
            scope.define_function(
                builder,
                "sdiv",
                &["uint256 x", "uint256 y"],
                Some("uint256"),
            );
            scope.define_function(builder, "selfdestruct", &["uint256 a"], None);
            scope.define_function(builder, "sgt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "signextend",
                &["uint256 i", "uint256 x"],
                Some("uint256"),
            );
            scope.define_function(builder, "sload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "slt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "smod",
                &["uint256 x", "uint256 y"],
                Some("uint256"),
            );
            scope.define_function(builder, "sstore", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "stop", &[], None);
            scope.define_function(builder, "sub", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "timestamp", &[], Some("uint256"));
            scope.define_function(builder, "xor", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "keccak256",
                &["uint256 p", "uint256 n"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "returndatacopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "returndatasize", &[], Some("uint256"));
            scope.define_function(
                builder,
                "staticcall",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "create2",
                &["uint256 v", "uint256 p", "uint256 n", "uint256 s"],
                Some("uint256"),
            );
            scope.define_function(builder, "extcodehash", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "sar", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "shl", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "shr", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "chainid", &[], Some("uint256"));
            scope.define_function(builder, "selfbalance", &[], Some("uint256"));
            scope.define_function(builder, "difficulty", &[], Some("uint256"));
        }
    } else if *version < Version::new(0, 6, 7) {
        // 0.6.2
        // SolidityBuiltIns
        {
            scope.define_function(
                builder,
                "addmod",
                &["uint x", "uint y", "uint k"],
                Some("uint256"),
            );
            scope.define_function(builder, "assert", &["bool condition"], None);
            scope.define_function(builder, "blockhash", &["uint blockNumber"], Some("bytes32"));
            scope.define_function(
                builder,
                "ecrecover",
                &["bytes32 hash", "uint8 v", "bytes32 r", "bytes32 s"],
                Some("address"),
            );
            scope.define_function(builder, "gasleft", &[], Some("uint256"));
            scope.define_function(builder, "keccak256", &["bytes memory"], Some("bytes32"));
            scope.define_function(builder, "log0", &["bytes32"], None);
            scope.define_function(builder, "log1", &["bytes32", "bytes32"], None);
            scope.define_function(builder, "log2", &["bytes32", "bytes32", "bytes32"], None);
            scope.define_function(
                builder,
                "log3",
                &["bytes32", "bytes32", "bytes32", "bytes32"],
                None,
            );
            scope.define_function(
                builder,
                "log4",
                &["bytes32", "bytes32", "bytes32", "bytes32", "bytes32"],
                None,
            );
            scope.define_function(
                builder,
                "mulmod",
                &["uint x", "uint y", "uint k"],
                Some("uint256"),
            );
            scope.define_function(builder, "require", &["bool condition"], None);
            scope.define_function(
                builder,
                "require",
                &["bool condition", "string memory message"],
                None,
            );
            scope.define_function(builder, "revert", &[], None);
            scope.define_function(builder, "revert", &["string memory reason"], None);
            scope.define_function(builder, "ripemd160", &["bytes memory"], Some("bytes20"));
            scope.define_function(
                builder,
                "selfdestruct",
                &["address payable recipient"],
                None,
            );
            scope.define_function(builder, "sha256", &["bytes memory"], Some("bytes32"));
            let mut type_scope = scope.define_type(builder, "%AbiType");
            type_scope.define_function(
                builder,
                "decode",
                &["bytes memory encodedData", "%Type[] encodedTypesTuple"],
                Some("%Any[]"),
            );
            type_scope.define_function(
                builder,
                "encode",
                &["%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodePacked",
                &["%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodeWithSelector",
                &["bytes4 selector", "%Any[] functionArgumentsTuple"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodeWithSignature",
                &["string memory signature", "%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "address");
            type_scope.define_field(builder, "balance", "uint256");
            type_scope.define_function(
                builder,
                "call",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(
                builder,
                "delegatecall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "send", &["uint256 amount"], Some("bool"));
            type_scope.define_function(
                builder,
                "staticcall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "address payable");
            type_scope.define_field(builder, "balance", "uint256");
            type_scope.define_function(
                builder,
                "call",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(
                builder,
                "delegatecall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "send", &["uint256 amount"], Some("bool"));
            type_scope.define_function(
                builder,
                "staticcall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "transfer", &["uint256 amount"], None);
            let mut type_scope = scope.define_type(builder, "%Array");
            type_scope.define_field(builder, "length", "uint256");
            type_scope.define_function(builder, "push", &[], Some("%ValueType"));
            type_scope.define_function(builder, "push", &["%ValueType element"], None);
            type_scope.define_function(builder, "pop", &[], None);
            let mut type_scope = scope.define_type(builder, "%FixedArray");
            type_scope.define_field(builder, "length", "uint256");
            let mut type_scope = scope.define_type(builder, "%BlockType");
            type_scope.define_field(builder, "coinbase", "address payable");
            type_scope.define_field(builder, "difficulty", "uint256");
            type_scope.define_field(builder, "gaslimit", "uint256");
            type_scope.define_field(builder, "number", "uint256");
            type_scope.define_field(builder, "timestamp", "uint256");
            let mut type_scope = scope.define_type(builder, "bytes");
            type_scope.define_field(builder, "length", "uint256");
            let mut type_scope = scope.define_type(builder, "%BytesType");
            type_scope.define_function(
                builder,
                "concat",
                &["bytes[] bytesToConcatenate"],
                Some("bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "%CallOptions");
            type_scope.define_field(builder, "gas", "uint256");
            type_scope.define_field(builder, "salt", "uint256");
            type_scope.define_field(builder, "value", "uint256");
            let mut type_scope = scope.define_type(builder, "Error");
            type_scope.define_field(builder, "reason", "string");
            let mut type_scope = scope.define_type(builder, "%Function");
            type_scope.define_function(builder, "gas", &["uint amount"], Some("function()"));
            type_scope.define_function(builder, "value", &["uint amount"], Some("function()"));
            let mut type_scope = scope.define_type(builder, "%ExternalFunction");
            type_scope.define_field(builder, "selector", "bytes4");
            type_scope.define_function(builder, "gas", &["uint amount"], Some("function()"));
            type_scope.define_function(builder, "value", &["uint amount"], Some("function()"));
            let mut type_scope = scope.define_type(builder, "%MessageType");
            type_scope.define_field(builder, "data", "bytes");
            type_scope.define_field(builder, "sender", "address payable");
            type_scope.define_field(builder, "sig", "bytes4");
            type_scope.define_field(builder, "value", "uint256");
            let mut type_scope = scope.define_type(builder, "Panic");
            type_scope.define_field(builder, "errorCode", "uint256");
            let mut type_scope = scope.define_type(builder, "%StringType");
            type_scope.define_function(
                builder,
                "concat",
                &["string[] stringsToConcatenate"],
                Some("string memory"),
            );
            let mut type_scope = scope.define_type(builder, "%TransactionType");
            type_scope.define_field(builder, "gasprice", "uint256");
            type_scope.define_field(builder, "origin", "address payable");
            let mut type_scope = scope.define_type(builder, "%ContractTypeType");
            type_scope.define_field(builder, "name", "string");
            type_scope.define_field(builder, "creationCode", "bytes");
            type_scope.define_field(builder, "runtimeCode", "bytes");
            let mut type_scope = scope.define_type(builder, "%InterfaceTypeType");
            type_scope.define_field(builder, "name", "string");
            _ = scope.define_type(builder, "%IntTypeType");
            scope.define_field(builder, "%_", "%Function");
            scope.define_field(builder, "abi", "%AbiType");
            scope.define_field(builder, "block", "%BlockType");
            scope.define_field(builder, "bytes", "%BytesType");
            scope.define_field(builder, "msg", "%MessageType");
            scope.define_field(builder, "now", "uint256");
            scope.define_field(builder, "string", "%StringType");
            scope.define_field(builder, "tx", "%TransactionType");
        }
        // YulBuiltIns
        {
            let mut scope = scope.new_context(builder, "@yul");
            let mut type_scope = scope.define_type(builder, "%YulExternal");
            type_scope.define_field(builder, "slot", "uint256");
            type_scope.define_field(builder, "offset", "uint256");
            type_scope.define_field(builder, "length", "uint256");
            scope.define_function(builder, "add", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "addmod",
                &["uint256 x", "uint256 y", "uint256 m"],
                Some("uint256"),
            );
            scope.define_function(builder, "address", &[], Some("uint256"));
            scope.define_function(builder, "and", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "balance", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "blockhash", &["uint256 b"], Some("uint256"));
            scope.define_function(
                builder,
                "byte",
                &["uint256 n", "uint256 x"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "callcode",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 v",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "calldatacopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "calldataload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "calldatasize", &[], Some("uint256"));
            scope.define_function(builder, "caller", &[], Some("uint256"));
            scope.define_function(
                builder,
                "call",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 v",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(builder, "callvalue", &[], Some("uint256"));
            scope.define_function(
                builder,
                "codecopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "codesize", &[], Some("uint256"));
            scope.define_function(builder, "coinbase", &[], Some("uint256"));
            scope.define_function(
                builder,
                "create",
                &["uint256 v", "uint256 p", "uint256 n"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "delegatecall",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(builder, "div", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "eq", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "exp", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "extcodecopy",
                &["uint256 a", "uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "extcodesize", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "gas", &[], Some("uint256"));
            scope.define_function(builder, "gaslimit", &[], Some("uint256"));
            scope.define_function(builder, "gasprice", &[], Some("uint256"));
            scope.define_function(builder, "gt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "invalid", &[], None);
            scope.define_function(builder, "iszero", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "log0", &["uint256 p", "uint256 s"], None);
            scope.define_function(
                builder,
                "log1",
                &["uint256 p", "uint256 s", "uint256 t1"],
                None,
            );
            scope.define_function(
                builder,
                "log2",
                &["uint256 p", "uint256 s", "uint256 t1", "uint256 t2"],
                None,
            );
            scope.define_function(
                builder,
                "log3",
                &[
                    "uint256 p",
                    "uint256 s",
                    "uint256 t1",
                    "uint256 t2",
                    "uint256 t3",
                ],
                None,
            );
            scope.define_function(
                builder,
                "log4",
                &[
                    "uint256 p",
                    "uint256 s",
                    "uint256 t1",
                    "uint256 t2",
                    "uint256 t3",
                ],
                None,
            );
            scope.define_function(builder, "lt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "mload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "mod", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "msize", &[], Some("uint256"));
            scope.define_function(builder, "mstore8", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "mstore", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "mul", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "mulmod",
                &["uint256 x", "uint256 y", "uint256 m"],
                Some("uint256"),
            );
            scope.define_function(builder, "not", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "number", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "origin", &[], Some("uint256"));
            scope.define_function(builder, "or", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "pop", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "return", &["uint256 p", "uint256 s"], None);
            scope.define_function(builder, "revert", &["uint256 p", "uint256 s"], None);
            scope.define_function(
                builder,
                "sdiv",
                &["uint256 x", "uint256 y"],
                Some("uint256"),
            );
            scope.define_function(builder, "selfdestruct", &["uint256 a"], None);
            scope.define_function(builder, "sgt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "signextend",
                &["uint256 i", "uint256 x"],
                Some("uint256"),
            );
            scope.define_function(builder, "sload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "slt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "smod",
                &["uint256 x", "uint256 y"],
                Some("uint256"),
            );
            scope.define_function(builder, "sstore", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "stop", &[], None);
            scope.define_function(builder, "sub", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "timestamp", &[], Some("uint256"));
            scope.define_function(builder, "xor", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "keccak256",
                &["uint256 p", "uint256 n"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "returndatacopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "returndatasize", &[], Some("uint256"));
            scope.define_function(
                builder,
                "staticcall",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "create2",
                &["uint256 v", "uint256 p", "uint256 n", "uint256 s"],
                Some("uint256"),
            );
            scope.define_function(builder, "extcodehash", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "sar", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "shl", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "shr", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "chainid", &[], Some("uint256"));
            scope.define_function(builder, "selfbalance", &[], Some("uint256"));
            scope.define_function(builder, "difficulty", &[], Some("uint256"));
        }
    } else if *version < Version::new(0, 6, 8) {
        // 0.6.7
        // SolidityBuiltIns
        {
            scope.define_function(
                builder,
                "addmod",
                &["uint x", "uint y", "uint k"],
                Some("uint256"),
            );
            scope.define_function(builder, "assert", &["bool condition"], None);
            scope.define_function(builder, "blockhash", &["uint blockNumber"], Some("bytes32"));
            scope.define_function(
                builder,
                "ecrecover",
                &["bytes32 hash", "uint8 v", "bytes32 r", "bytes32 s"],
                Some("address"),
            );
            scope.define_function(builder, "gasleft", &[], Some("uint256"));
            scope.define_function(builder, "keccak256", &["bytes memory"], Some("bytes32"));
            scope.define_function(builder, "log0", &["bytes32"], None);
            scope.define_function(builder, "log1", &["bytes32", "bytes32"], None);
            scope.define_function(builder, "log2", &["bytes32", "bytes32", "bytes32"], None);
            scope.define_function(
                builder,
                "log3",
                &["bytes32", "bytes32", "bytes32", "bytes32"],
                None,
            );
            scope.define_function(
                builder,
                "log4",
                &["bytes32", "bytes32", "bytes32", "bytes32", "bytes32"],
                None,
            );
            scope.define_function(
                builder,
                "mulmod",
                &["uint x", "uint y", "uint k"],
                Some("uint256"),
            );
            scope.define_function(builder, "require", &["bool condition"], None);
            scope.define_function(
                builder,
                "require",
                &["bool condition", "string memory message"],
                None,
            );
            scope.define_function(builder, "revert", &[], None);
            scope.define_function(builder, "revert", &["string memory reason"], None);
            scope.define_function(builder, "ripemd160", &["bytes memory"], Some("bytes20"));
            scope.define_function(
                builder,
                "selfdestruct",
                &["address payable recipient"],
                None,
            );
            scope.define_function(builder, "sha256", &["bytes memory"], Some("bytes32"));
            let mut type_scope = scope.define_type(builder, "%AbiType");
            type_scope.define_function(
                builder,
                "decode",
                &["bytes memory encodedData", "%Type[] encodedTypesTuple"],
                Some("%Any[]"),
            );
            type_scope.define_function(
                builder,
                "encode",
                &["%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodePacked",
                &["%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodeWithSelector",
                &["bytes4 selector", "%Any[] functionArgumentsTuple"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodeWithSignature",
                &["string memory signature", "%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "address");
            type_scope.define_field(builder, "balance", "uint256");
            type_scope.define_function(
                builder,
                "call",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(
                builder,
                "delegatecall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "send", &["uint256 amount"], Some("bool"));
            type_scope.define_function(
                builder,
                "staticcall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "address payable");
            type_scope.define_field(builder, "balance", "uint256");
            type_scope.define_function(
                builder,
                "call",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(
                builder,
                "delegatecall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "send", &["uint256 amount"], Some("bool"));
            type_scope.define_function(
                builder,
                "staticcall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "transfer", &["uint256 amount"], None);
            let mut type_scope = scope.define_type(builder, "%Array");
            type_scope.define_field(builder, "length", "uint256");
            type_scope.define_function(builder, "push", &[], Some("%ValueType"));
            type_scope.define_function(builder, "push", &["%ValueType element"], None);
            type_scope.define_function(builder, "pop", &[], None);
            let mut type_scope = scope.define_type(builder, "%FixedArray");
            type_scope.define_field(builder, "length", "uint256");
            let mut type_scope = scope.define_type(builder, "%BlockType");
            type_scope.define_field(builder, "coinbase", "address payable");
            type_scope.define_field(builder, "difficulty", "uint256");
            type_scope.define_field(builder, "gaslimit", "uint256");
            type_scope.define_field(builder, "number", "uint256");
            type_scope.define_field(builder, "timestamp", "uint256");
            let mut type_scope = scope.define_type(builder, "bytes");
            type_scope.define_field(builder, "length", "uint256");
            let mut type_scope = scope.define_type(builder, "%BytesType");
            type_scope.define_function(
                builder,
                "concat",
                &["bytes[] bytesToConcatenate"],
                Some("bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "%CallOptions");
            type_scope.define_field(builder, "gas", "uint256");
            type_scope.define_field(builder, "salt", "uint256");
            type_scope.define_field(builder, "value", "uint256");
            let mut type_scope = scope.define_type(builder, "Error");
            type_scope.define_field(builder, "reason", "string");
            let mut type_scope = scope.define_type(builder, "%Function");
            type_scope.define_function(builder, "gas", &["uint amount"], Some("function()"));
            type_scope.define_function(builder, "value", &["uint amount"], Some("function()"));
            let mut type_scope = scope.define_type(builder, "%ExternalFunction");
            type_scope.define_field(builder, "selector", "bytes4");
            type_scope.define_function(builder, "gas", &["uint amount"], Some("function()"));
            type_scope.define_function(builder, "value", &["uint amount"], Some("function()"));
            let mut type_scope = scope.define_type(builder, "%MessageType");
            type_scope.define_field(builder, "data", "bytes");
            type_scope.define_field(builder, "sender", "address payable");
            type_scope.define_field(builder, "sig", "bytes4");
            type_scope.define_field(builder, "value", "uint256");
            let mut type_scope = scope.define_type(builder, "Panic");
            type_scope.define_field(builder, "errorCode", "uint256");
            let mut type_scope = scope.define_type(builder, "%StringType");
            type_scope.define_function(
                builder,
                "concat",
                &["string[] stringsToConcatenate"],
                Some("string memory"),
            );
            let mut type_scope = scope.define_type(builder, "%TransactionType");
            type_scope.define_field(builder, "gasprice", "uint256");
            type_scope.define_field(builder, "origin", "address payable");
            let mut type_scope = scope.define_type(builder, "%ContractTypeType");
            type_scope.define_field(builder, "name", "string");
            type_scope.define_field(builder, "creationCode", "bytes");
            type_scope.define_field(builder, "runtimeCode", "bytes");
            type_scope.define_field(builder, "interfaceId", "bytes4");
            let mut type_scope = scope.define_type(builder, "%InterfaceTypeType");
            type_scope.define_field(builder, "name", "string");
            type_scope.define_field(builder, "interfaceId", "bytes4");
            _ = scope.define_type(builder, "%IntTypeType");
            scope.define_field(builder, "%_", "%Function");
            scope.define_field(builder, "abi", "%AbiType");
            scope.define_field(builder, "block", "%BlockType");
            scope.define_field(builder, "bytes", "%BytesType");
            scope.define_field(builder, "msg", "%MessageType");
            scope.define_field(builder, "now", "uint256");
            scope.define_field(builder, "string", "%StringType");
            scope.define_field(builder, "tx", "%TransactionType");
        }
        // YulBuiltIns
        {
            let mut scope = scope.new_context(builder, "@yul");
            let mut type_scope = scope.define_type(builder, "%YulExternal");
            type_scope.define_field(builder, "slot", "uint256");
            type_scope.define_field(builder, "offset", "uint256");
            type_scope.define_field(builder, "length", "uint256");
            scope.define_function(builder, "add", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "addmod",
                &["uint256 x", "uint256 y", "uint256 m"],
                Some("uint256"),
            );
            scope.define_function(builder, "address", &[], Some("uint256"));
            scope.define_function(builder, "and", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "balance", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "blockhash", &["uint256 b"], Some("uint256"));
            scope.define_function(
                builder,
                "byte",
                &["uint256 n", "uint256 x"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "callcode",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 v",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "calldatacopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "calldataload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "calldatasize", &[], Some("uint256"));
            scope.define_function(builder, "caller", &[], Some("uint256"));
            scope.define_function(
                builder,
                "call",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 v",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(builder, "callvalue", &[], Some("uint256"));
            scope.define_function(
                builder,
                "codecopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "codesize", &[], Some("uint256"));
            scope.define_function(builder, "coinbase", &[], Some("uint256"));
            scope.define_function(
                builder,
                "create",
                &["uint256 v", "uint256 p", "uint256 n"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "delegatecall",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(builder, "div", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "eq", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "exp", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "extcodecopy",
                &["uint256 a", "uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "extcodesize", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "gas", &[], Some("uint256"));
            scope.define_function(builder, "gaslimit", &[], Some("uint256"));
            scope.define_function(builder, "gasprice", &[], Some("uint256"));
            scope.define_function(builder, "gt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "invalid", &[], None);
            scope.define_function(builder, "iszero", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "log0", &["uint256 p", "uint256 s"], None);
            scope.define_function(
                builder,
                "log1",
                &["uint256 p", "uint256 s", "uint256 t1"],
                None,
            );
            scope.define_function(
                builder,
                "log2",
                &["uint256 p", "uint256 s", "uint256 t1", "uint256 t2"],
                None,
            );
            scope.define_function(
                builder,
                "log3",
                &[
                    "uint256 p",
                    "uint256 s",
                    "uint256 t1",
                    "uint256 t2",
                    "uint256 t3",
                ],
                None,
            );
            scope.define_function(
                builder,
                "log4",
                &[
                    "uint256 p",
                    "uint256 s",
                    "uint256 t1",
                    "uint256 t2",
                    "uint256 t3",
                ],
                None,
            );
            scope.define_function(builder, "lt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "mload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "mod", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "msize", &[], Some("uint256"));
            scope.define_function(builder, "mstore8", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "mstore", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "mul", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "mulmod",
                &["uint256 x", "uint256 y", "uint256 m"],
                Some("uint256"),
            );
            scope.define_function(builder, "not", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "number", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "origin", &[], Some("uint256"));
            scope.define_function(builder, "or", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "pop", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "return", &["uint256 p", "uint256 s"], None);
            scope.define_function(builder, "revert", &["uint256 p", "uint256 s"], None);
            scope.define_function(
                builder,
                "sdiv",
                &["uint256 x", "uint256 y"],
                Some("uint256"),
            );
            scope.define_function(builder, "selfdestruct", &["uint256 a"], None);
            scope.define_function(builder, "sgt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "signextend",
                &["uint256 i", "uint256 x"],
                Some("uint256"),
            );
            scope.define_function(builder, "sload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "slt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "smod",
                &["uint256 x", "uint256 y"],
                Some("uint256"),
            );
            scope.define_function(builder, "sstore", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "stop", &[], None);
            scope.define_function(builder, "sub", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "timestamp", &[], Some("uint256"));
            scope.define_function(builder, "xor", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "keccak256",
                &["uint256 p", "uint256 n"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "returndatacopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "returndatasize", &[], Some("uint256"));
            scope.define_function(
                builder,
                "staticcall",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "create2",
                &["uint256 v", "uint256 p", "uint256 n", "uint256 s"],
                Some("uint256"),
            );
            scope.define_function(builder, "extcodehash", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "sar", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "shl", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "shr", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "chainid", &[], Some("uint256"));
            scope.define_function(builder, "selfbalance", &[], Some("uint256"));
            scope.define_function(builder, "difficulty", &[], Some("uint256"));
        }
    } else if *version < Version::new(0, 7, 0) {
        // 0.6.8
        // SolidityBuiltIns
        {
            scope.define_function(
                builder,
                "addmod",
                &["uint x", "uint y", "uint k"],
                Some("uint256"),
            );
            scope.define_function(builder, "assert", &["bool condition"], None);
            scope.define_function(builder, "blockhash", &["uint blockNumber"], Some("bytes32"));
            scope.define_function(
                builder,
                "ecrecover",
                &["bytes32 hash", "uint8 v", "bytes32 r", "bytes32 s"],
                Some("address"),
            );
            scope.define_function(builder, "gasleft", &[], Some("uint256"));
            scope.define_function(builder, "keccak256", &["bytes memory"], Some("bytes32"));
            scope.define_function(builder, "log0", &["bytes32"], None);
            scope.define_function(builder, "log1", &["bytes32", "bytes32"], None);
            scope.define_function(builder, "log2", &["bytes32", "bytes32", "bytes32"], None);
            scope.define_function(
                builder,
                "log3",
                &["bytes32", "bytes32", "bytes32", "bytes32"],
                None,
            );
            scope.define_function(
                builder,
                "log4",
                &["bytes32", "bytes32", "bytes32", "bytes32", "bytes32"],
                None,
            );
            scope.define_function(
                builder,
                "mulmod",
                &["uint x", "uint y", "uint k"],
                Some("uint256"),
            );
            scope.define_function(builder, "require", &["bool condition"], None);
            scope.define_function(
                builder,
                "require",
                &["bool condition", "string memory message"],
                None,
            );
            scope.define_function(builder, "revert", &[], None);
            scope.define_function(builder, "revert", &["string memory reason"], None);
            scope.define_function(builder, "ripemd160", &["bytes memory"], Some("bytes20"));
            scope.define_function(
                builder,
                "selfdestruct",
                &["address payable recipient"],
                None,
            );
            scope.define_function(builder, "sha256", &["bytes memory"], Some("bytes32"));
            let mut type_scope = scope.define_type(builder, "%AbiType");
            type_scope.define_function(
                builder,
                "decode",
                &["bytes memory encodedData", "%Type[] encodedTypesTuple"],
                Some("%Any[]"),
            );
            type_scope.define_function(
                builder,
                "encode",
                &["%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodePacked",
                &["%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodeWithSelector",
                &["bytes4 selector", "%Any[] functionArgumentsTuple"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodeWithSignature",
                &["string memory signature", "%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "address");
            type_scope.define_field(builder, "balance", "uint256");
            type_scope.define_function(
                builder,
                "call",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(
                builder,
                "delegatecall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "send", &["uint256 amount"], Some("bool"));
            type_scope.define_function(
                builder,
                "staticcall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "address payable");
            type_scope.define_field(builder, "balance", "uint256");
            type_scope.define_function(
                builder,
                "call",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(
                builder,
                "delegatecall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "send", &["uint256 amount"], Some("bool"));
            type_scope.define_function(
                builder,
                "staticcall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "transfer", &["uint256 amount"], None);
            let mut type_scope = scope.define_type(builder, "%Array");
            type_scope.define_field(builder, "length", "uint256");
            type_scope.define_function(builder, "push", &[], Some("%ValueType"));
            type_scope.define_function(builder, "push", &["%ValueType element"], None);
            type_scope.define_function(builder, "pop", &[], None);
            let mut type_scope = scope.define_type(builder, "%FixedArray");
            type_scope.define_field(builder, "length", "uint256");
            let mut type_scope = scope.define_type(builder, "%BlockType");
            type_scope.define_field(builder, "coinbase", "address payable");
            type_scope.define_field(builder, "difficulty", "uint256");
            type_scope.define_field(builder, "gaslimit", "uint256");
            type_scope.define_field(builder, "number", "uint256");
            type_scope.define_field(builder, "timestamp", "uint256");
            let mut type_scope = scope.define_type(builder, "bytes");
            type_scope.define_field(builder, "length", "uint256");
            let mut type_scope = scope.define_type(builder, "%BytesType");
            type_scope.define_function(
                builder,
                "concat",
                &["bytes[] bytesToConcatenate"],
                Some("bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "%CallOptions");
            type_scope.define_field(builder, "gas", "uint256");
            type_scope.define_field(builder, "salt", "uint256");
            type_scope.define_field(builder, "value", "uint256");
            let mut type_scope = scope.define_type(builder, "Error");
            type_scope.define_field(builder, "reason", "string");
            let mut type_scope = scope.define_type(builder, "%Function");
            type_scope.define_function(builder, "gas", &["uint amount"], Some("function()"));
            type_scope.define_function(builder, "value", &["uint amount"], Some("function()"));
            let mut type_scope = scope.define_type(builder, "%ExternalFunction");
            type_scope.define_field(builder, "selector", "bytes4");
            type_scope.define_function(builder, "gas", &["uint amount"], Some("function()"));
            type_scope.define_function(builder, "value", &["uint amount"], Some("function()"));
            let mut type_scope = scope.define_type(builder, "%MessageType");
            type_scope.define_field(builder, "data", "bytes");
            type_scope.define_field(builder, "sender", "address payable");
            type_scope.define_field(builder, "sig", "bytes4");
            type_scope.define_field(builder, "value", "uint256");
            let mut type_scope = scope.define_type(builder, "Panic");
            type_scope.define_field(builder, "errorCode", "uint256");
            let mut type_scope = scope.define_type(builder, "%StringType");
            type_scope.define_function(
                builder,
                "concat",
                &["string[] stringsToConcatenate"],
                Some("string memory"),
            );
            let mut type_scope = scope.define_type(builder, "%TransactionType");
            type_scope.define_field(builder, "gasprice", "uint256");
            type_scope.define_field(builder, "origin", "address payable");
            let mut type_scope = scope.define_type(builder, "%ContractTypeType");
            type_scope.define_field(builder, "name", "string");
            type_scope.define_field(builder, "creationCode", "bytes");
            type_scope.define_field(builder, "runtimeCode", "bytes");
            type_scope.define_field(builder, "interfaceId", "bytes4");
            let mut type_scope = scope.define_type(builder, "%InterfaceTypeType");
            type_scope.define_field(builder, "name", "string");
            type_scope.define_field(builder, "interfaceId", "bytes4");
            let mut type_scope = scope.define_type(builder, "%IntTypeType");
            type_scope.define_field(builder, "min", "int256");
            type_scope.define_field(builder, "max", "int256");
            scope.define_field(builder, "%_", "%Function");
            scope.define_field(builder, "abi", "%AbiType");
            scope.define_field(builder, "block", "%BlockType");
            scope.define_field(builder, "bytes", "%BytesType");
            scope.define_field(builder, "msg", "%MessageType");
            scope.define_field(builder, "now", "uint256");
            scope.define_field(builder, "string", "%StringType");
            scope.define_field(builder, "tx", "%TransactionType");
        }
        // YulBuiltIns
        {
            let mut scope = scope.new_context(builder, "@yul");
            let mut type_scope = scope.define_type(builder, "%YulExternal");
            type_scope.define_field(builder, "slot", "uint256");
            type_scope.define_field(builder, "offset", "uint256");
            type_scope.define_field(builder, "length", "uint256");
            scope.define_function(builder, "add", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "addmod",
                &["uint256 x", "uint256 y", "uint256 m"],
                Some("uint256"),
            );
            scope.define_function(builder, "address", &[], Some("uint256"));
            scope.define_function(builder, "and", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "balance", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "blockhash", &["uint256 b"], Some("uint256"));
            scope.define_function(
                builder,
                "byte",
                &["uint256 n", "uint256 x"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "callcode",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 v",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "calldatacopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "calldataload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "calldatasize", &[], Some("uint256"));
            scope.define_function(builder, "caller", &[], Some("uint256"));
            scope.define_function(
                builder,
                "call",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 v",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(builder, "callvalue", &[], Some("uint256"));
            scope.define_function(
                builder,
                "codecopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "codesize", &[], Some("uint256"));
            scope.define_function(builder, "coinbase", &[], Some("uint256"));
            scope.define_function(
                builder,
                "create",
                &["uint256 v", "uint256 p", "uint256 n"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "delegatecall",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(builder, "div", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "eq", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "exp", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "extcodecopy",
                &["uint256 a", "uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "extcodesize", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "gas", &[], Some("uint256"));
            scope.define_function(builder, "gaslimit", &[], Some("uint256"));
            scope.define_function(builder, "gasprice", &[], Some("uint256"));
            scope.define_function(builder, "gt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "invalid", &[], None);
            scope.define_function(builder, "iszero", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "log0", &["uint256 p", "uint256 s"], None);
            scope.define_function(
                builder,
                "log1",
                &["uint256 p", "uint256 s", "uint256 t1"],
                None,
            );
            scope.define_function(
                builder,
                "log2",
                &["uint256 p", "uint256 s", "uint256 t1", "uint256 t2"],
                None,
            );
            scope.define_function(
                builder,
                "log3",
                &[
                    "uint256 p",
                    "uint256 s",
                    "uint256 t1",
                    "uint256 t2",
                    "uint256 t3",
                ],
                None,
            );
            scope.define_function(
                builder,
                "log4",
                &[
                    "uint256 p",
                    "uint256 s",
                    "uint256 t1",
                    "uint256 t2",
                    "uint256 t3",
                ],
                None,
            );
            scope.define_function(builder, "lt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "mload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "mod", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "msize", &[], Some("uint256"));
            scope.define_function(builder, "mstore8", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "mstore", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "mul", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "mulmod",
                &["uint256 x", "uint256 y", "uint256 m"],
                Some("uint256"),
            );
            scope.define_function(builder, "not", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "number", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "origin", &[], Some("uint256"));
            scope.define_function(builder, "or", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "pop", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "return", &["uint256 p", "uint256 s"], None);
            scope.define_function(builder, "revert", &["uint256 p", "uint256 s"], None);
            scope.define_function(
                builder,
                "sdiv",
                &["uint256 x", "uint256 y"],
                Some("uint256"),
            );
            scope.define_function(builder, "selfdestruct", &["uint256 a"], None);
            scope.define_function(builder, "sgt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "signextend",
                &["uint256 i", "uint256 x"],
                Some("uint256"),
            );
            scope.define_function(builder, "sload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "slt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "smod",
                &["uint256 x", "uint256 y"],
                Some("uint256"),
            );
            scope.define_function(builder, "sstore", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "stop", &[], None);
            scope.define_function(builder, "sub", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "timestamp", &[], Some("uint256"));
            scope.define_function(builder, "xor", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "keccak256",
                &["uint256 p", "uint256 n"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "returndatacopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "returndatasize", &[], Some("uint256"));
            scope.define_function(
                builder,
                "staticcall",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "create2",
                &["uint256 v", "uint256 p", "uint256 n", "uint256 s"],
                Some("uint256"),
            );
            scope.define_function(builder, "extcodehash", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "sar", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "shl", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "shr", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "chainid", &[], Some("uint256"));
            scope.define_function(builder, "selfbalance", &[], Some("uint256"));
            scope.define_function(builder, "difficulty", &[], Some("uint256"));
        }
    } else if *version < Version::new(0, 8, 0) {
        // 0.7.0
        // SolidityBuiltIns
        {
            scope.define_function(
                builder,
                "addmod",
                &["uint x", "uint y", "uint k"],
                Some("uint256"),
            );
            scope.define_function(builder, "assert", &["bool condition"], None);
            scope.define_function(builder, "blockhash", &["uint blockNumber"], Some("bytes32"));
            scope.define_function(
                builder,
                "ecrecover",
                &["bytes32 hash", "uint8 v", "bytes32 r", "bytes32 s"],
                Some("address"),
            );
            scope.define_function(builder, "gasleft", &[], Some("uint256"));
            scope.define_function(builder, "keccak256", &["bytes memory"], Some("bytes32"));
            scope.define_function(builder, "log0", &["bytes32"], None);
            scope.define_function(builder, "log1", &["bytes32", "bytes32"], None);
            scope.define_function(builder, "log2", &["bytes32", "bytes32", "bytes32"], None);
            scope.define_function(
                builder,
                "log3",
                &["bytes32", "bytes32", "bytes32", "bytes32"],
                None,
            );
            scope.define_function(
                builder,
                "log4",
                &["bytes32", "bytes32", "bytes32", "bytes32", "bytes32"],
                None,
            );
            scope.define_function(
                builder,
                "mulmod",
                &["uint x", "uint y", "uint k"],
                Some("uint256"),
            );
            scope.define_function(builder, "require", &["bool condition"], None);
            scope.define_function(
                builder,
                "require",
                &["bool condition", "string memory message"],
                None,
            );
            scope.define_function(builder, "revert", &[], None);
            scope.define_function(builder, "revert", &["string memory reason"], None);
            scope.define_function(builder, "ripemd160", &["bytes memory"], Some("bytes20"));
            scope.define_function(
                builder,
                "selfdestruct",
                &["address payable recipient"],
                None,
            );
            scope.define_function(builder, "sha256", &["bytes memory"], Some("bytes32"));
            let mut type_scope = scope.define_type(builder, "%AbiType");
            type_scope.define_function(
                builder,
                "decode",
                &["bytes memory encodedData", "%Type[] encodedTypesTuple"],
                Some("%Any[]"),
            );
            type_scope.define_function(
                builder,
                "encode",
                &["%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodePacked",
                &["%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodeWithSelector",
                &["bytes4 selector", "%Any[] functionArgumentsTuple"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodeWithSignature",
                &["string memory signature", "%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "address");
            type_scope.define_field(builder, "balance", "uint256");
            type_scope.define_function(
                builder,
                "call",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(
                builder,
                "delegatecall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "send", &["uint256 amount"], Some("bool"));
            type_scope.define_function(
                builder,
                "staticcall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "address payable");
            type_scope.define_field(builder, "balance", "uint256");
            type_scope.define_function(
                builder,
                "call",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(
                builder,
                "delegatecall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "send", &["uint256 amount"], Some("bool"));
            type_scope.define_function(
                builder,
                "staticcall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "transfer", &["uint256 amount"], None);
            let mut type_scope = scope.define_type(builder, "%Array");
            type_scope.define_field(builder, "length", "uint256");
            type_scope.define_function(builder, "push", &[], Some("%ValueType"));
            type_scope.define_function(builder, "push", &["%ValueType element"], None);
            type_scope.define_function(builder, "pop", &[], None);
            let mut type_scope = scope.define_type(builder, "%FixedArray");
            type_scope.define_field(builder, "length", "uint256");
            let mut type_scope = scope.define_type(builder, "%BlockType");
            type_scope.define_field(builder, "coinbase", "address payable");
            type_scope.define_field(builder, "difficulty", "uint256");
            type_scope.define_field(builder, "gaslimit", "uint256");
            type_scope.define_field(builder, "number", "uint256");
            type_scope.define_field(builder, "timestamp", "uint256");
            let mut type_scope = scope.define_type(builder, "bytes");
            type_scope.define_field(builder, "length", "uint256");
            let mut type_scope = scope.define_type(builder, "%BytesType");
            type_scope.define_function(
                builder,
                "concat",
                &["bytes[] bytesToConcatenate"],
                Some("bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "%CallOptions");
            type_scope.define_field(builder, "gas", "uint256");
            type_scope.define_field(builder, "salt", "uint256");
            type_scope.define_field(builder, "value", "uint256");
            let mut type_scope = scope.define_type(builder, "Error");
            type_scope.define_field(builder, "reason", "string");
            _ = scope.define_type(builder, "%Function");
            let mut type_scope = scope.define_type(builder, "%ExternalFunction");
            type_scope.define_field(builder, "selector", "bytes4");
            let mut type_scope = scope.define_type(builder, "%MessageType");
            type_scope.define_field(builder, "data", "bytes");
            type_scope.define_field(builder, "sender", "address payable");
            type_scope.define_field(builder, "sig", "bytes4");
            type_scope.define_field(builder, "value", "uint256");
            let mut type_scope = scope.define_type(builder, "Panic");
            type_scope.define_field(builder, "errorCode", "uint256");
            let mut type_scope = scope.define_type(builder, "%StringType");
            type_scope.define_function(
                builder,
                "concat",
                &["string[] stringsToConcatenate"],
                Some("string memory"),
            );
            let mut type_scope = scope.define_type(builder, "%TransactionType");
            type_scope.define_field(builder, "gasprice", "uint256");
            type_scope.define_field(builder, "origin", "address payable");
            let mut type_scope = scope.define_type(builder, "%ContractTypeType");
            type_scope.define_field(builder, "name", "string");
            type_scope.define_field(builder, "creationCode", "bytes");
            type_scope.define_field(builder, "runtimeCode", "bytes");
            type_scope.define_field(builder, "interfaceId", "bytes4");
            let mut type_scope = scope.define_type(builder, "%InterfaceTypeType");
            type_scope.define_field(builder, "name", "string");
            type_scope.define_field(builder, "interfaceId", "bytes4");
            let mut type_scope = scope.define_type(builder, "%IntTypeType");
            type_scope.define_field(builder, "min", "int256");
            type_scope.define_field(builder, "max", "int256");
            scope.define_field(builder, "%_", "%Function");
            scope.define_field(builder, "abi", "%AbiType");
            scope.define_field(builder, "block", "%BlockType");
            scope.define_field(builder, "bytes", "%BytesType");
            scope.define_field(builder, "msg", "%MessageType");
            scope.define_field(builder, "string", "%StringType");
            scope.define_field(builder, "tx", "%TransactionType");
        }
        // YulBuiltIns
        {
            let mut scope = scope.new_context(builder, "@yul");
            let mut type_scope = scope.define_type(builder, "%YulExternal");
            type_scope.define_field(builder, "slot", "uint256");
            type_scope.define_field(builder, "offset", "uint256");
            type_scope.define_field(builder, "length", "uint256");
            scope.define_function(builder, "add", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "addmod",
                &["uint256 x", "uint256 y", "uint256 m"],
                Some("uint256"),
            );
            scope.define_function(builder, "address", &[], Some("uint256"));
            scope.define_function(builder, "and", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "balance", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "blockhash", &["uint256 b"], Some("uint256"));
            scope.define_function(
                builder,
                "byte",
                &["uint256 n", "uint256 x"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "callcode",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 v",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "calldatacopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "calldataload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "calldatasize", &[], Some("uint256"));
            scope.define_function(builder, "caller", &[], Some("uint256"));
            scope.define_function(
                builder,
                "call",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 v",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(builder, "callvalue", &[], Some("uint256"));
            scope.define_function(
                builder,
                "codecopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "codesize", &[], Some("uint256"));
            scope.define_function(builder, "coinbase", &[], Some("uint256"));
            scope.define_function(
                builder,
                "create",
                &["uint256 v", "uint256 p", "uint256 n"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "delegatecall",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(builder, "div", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "eq", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "exp", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "extcodecopy",
                &["uint256 a", "uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "extcodesize", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "gas", &[], Some("uint256"));
            scope.define_function(builder, "gaslimit", &[], Some("uint256"));
            scope.define_function(builder, "gasprice", &[], Some("uint256"));
            scope.define_function(builder, "gt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "invalid", &[], None);
            scope.define_function(builder, "iszero", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "log0", &["uint256 p", "uint256 s"], None);
            scope.define_function(
                builder,
                "log1",
                &["uint256 p", "uint256 s", "uint256 t1"],
                None,
            );
            scope.define_function(
                builder,
                "log2",
                &["uint256 p", "uint256 s", "uint256 t1", "uint256 t2"],
                None,
            );
            scope.define_function(
                builder,
                "log3",
                &[
                    "uint256 p",
                    "uint256 s",
                    "uint256 t1",
                    "uint256 t2",
                    "uint256 t3",
                ],
                None,
            );
            scope.define_function(
                builder,
                "log4",
                &[
                    "uint256 p",
                    "uint256 s",
                    "uint256 t1",
                    "uint256 t2",
                    "uint256 t3",
                ],
                None,
            );
            scope.define_function(builder, "lt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "mload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "mod", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "msize", &[], Some("uint256"));
            scope.define_function(builder, "mstore8", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "mstore", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "mul", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "mulmod",
                &["uint256 x", "uint256 y", "uint256 m"],
                Some("uint256"),
            );
            scope.define_function(builder, "not", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "number", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "origin", &[], Some("uint256"));
            scope.define_function(builder, "or", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "pop", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "return", &["uint256 p", "uint256 s"], None);
            scope.define_function(builder, "revert", &["uint256 p", "uint256 s"], None);
            scope.define_function(
                builder,
                "sdiv",
                &["uint256 x", "uint256 y"],
                Some("uint256"),
            );
            scope.define_function(builder, "selfdestruct", &["uint256 a"], None);
            scope.define_function(builder, "sgt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "signextend",
                &["uint256 i", "uint256 x"],
                Some("uint256"),
            );
            scope.define_function(builder, "sload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "slt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "smod",
                &["uint256 x", "uint256 y"],
                Some("uint256"),
            );
            scope.define_function(builder, "sstore", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "stop", &[], None);
            scope.define_function(builder, "sub", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "timestamp", &[], Some("uint256"));
            scope.define_function(builder, "xor", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "keccak256",
                &["uint256 p", "uint256 n"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "returndatacopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "returndatasize", &[], Some("uint256"));
            scope.define_function(
                builder,
                "staticcall",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "create2",
                &["uint256 v", "uint256 p", "uint256 n", "uint256 s"],
                Some("uint256"),
            );
            scope.define_function(builder, "extcodehash", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "sar", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "shl", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "shr", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "chainid", &[], Some("uint256"));
            scope.define_function(builder, "selfbalance", &[], Some("uint256"));
            scope.define_function(builder, "difficulty", &[], Some("uint256"));
        }
    } else if *version < Version::new(0, 8, 2) {
        // 0.8.0
        // SolidityBuiltIns
        {
            scope.define_function(
                builder,
                "addmod",
                &["uint x", "uint y", "uint k"],
                Some("uint256"),
            );
            scope.define_function(builder, "assert", &["bool condition"], None);
            scope.define_function(builder, "blockhash", &["uint blockNumber"], Some("bytes32"));
            scope.define_function(
                builder,
                "ecrecover",
                &["bytes32 hash", "uint8 v", "bytes32 r", "bytes32 s"],
                Some("address"),
            );
            scope.define_function(builder, "gasleft", &[], Some("uint256"));
            scope.define_function(builder, "keccak256", &["bytes memory"], Some("bytes32"));
            scope.define_function(
                builder,
                "mulmod",
                &["uint x", "uint y", "uint k"],
                Some("uint256"),
            );
            scope.define_function(builder, "require", &["bool condition"], None);
            scope.define_function(
                builder,
                "require",
                &["bool condition", "string memory message"],
                None,
            );
            scope.define_function(builder, "revert", &[], None);
            scope.define_function(builder, "revert", &["string memory reason"], None);
            scope.define_function(builder, "ripemd160", &["bytes memory"], Some("bytes20"));
            scope.define_function(
                builder,
                "selfdestruct",
                &["address payable recipient"],
                None,
            );
            scope.define_function(builder, "sha256", &["bytes memory"], Some("bytes32"));
            let mut type_scope = scope.define_type(builder, "%AbiType");
            type_scope.define_function(
                builder,
                "decode",
                &["bytes memory encodedData", "%Type[] encodedTypesTuple"],
                Some("%Any[]"),
            );
            type_scope.define_function(
                builder,
                "encode",
                &["%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodePacked",
                &["%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodeWithSelector",
                &["bytes4 selector", "%Any[] functionArgumentsTuple"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodeWithSignature",
                &["string memory signature", "%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "address");
            type_scope.define_field(builder, "balance", "uint256");
            type_scope.define_field(builder, "code", "bytes");
            type_scope.define_field(builder, "codehash", "bytes32");
            type_scope.define_function(
                builder,
                "call",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(
                builder,
                "delegatecall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "send", &["uint256 amount"], Some("bool"));
            type_scope.define_function(
                builder,
                "staticcall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "address payable");
            type_scope.define_field(builder, "balance", "uint256");
            type_scope.define_field(builder, "code", "bytes");
            type_scope.define_field(builder, "codehash", "bytes32");
            type_scope.define_function(
                builder,
                "call",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(
                builder,
                "delegatecall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "send", &["uint256 amount"], Some("bool"));
            type_scope.define_function(
                builder,
                "staticcall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "transfer", &["uint256 amount"], None);
            let mut type_scope = scope.define_type(builder, "%Array");
            type_scope.define_field(builder, "length", "uint256");
            type_scope.define_function(builder, "push", &[], Some("%ValueType"));
            type_scope.define_function(builder, "push", &["%ValueType element"], None);
            type_scope.define_function(builder, "pop", &[], None);
            let mut type_scope = scope.define_type(builder, "%FixedArray");
            type_scope.define_field(builder, "length", "uint256");
            let mut type_scope = scope.define_type(builder, "%BlockType");
            type_scope.define_field(builder, "chainid", "uint256");
            type_scope.define_field(builder, "coinbase", "address payable");
            type_scope.define_field(builder, "difficulty", "uint256");
            type_scope.define_field(builder, "gaslimit", "uint256");
            type_scope.define_field(builder, "number", "uint256");
            type_scope.define_field(builder, "timestamp", "uint256");
            let mut type_scope = scope.define_type(builder, "bytes");
            type_scope.define_field(builder, "length", "uint256");
            let mut type_scope = scope.define_type(builder, "%BytesType");
            type_scope.define_function(
                builder,
                "concat",
                &["bytes[] bytesToConcatenate"],
                Some("bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "%CallOptions");
            type_scope.define_field(builder, "gas", "uint256");
            type_scope.define_field(builder, "salt", "uint256");
            type_scope.define_field(builder, "value", "uint256");
            let mut type_scope = scope.define_type(builder, "Error");
            type_scope.define_field(builder, "reason", "string");
            _ = scope.define_type(builder, "%Function");
            let mut type_scope = scope.define_type(builder, "%ExternalFunction");
            type_scope.define_field(builder, "selector", "bytes4");
            let mut type_scope = scope.define_type(builder, "%MessageType");
            type_scope.define_field(builder, "data", "bytes");
            type_scope.define_field(builder, "sender", "address");
            type_scope.define_field(builder, "sig", "bytes4");
            type_scope.define_field(builder, "value", "uint256");
            let mut type_scope = scope.define_type(builder, "Panic");
            type_scope.define_field(builder, "errorCode", "uint256");
            let mut type_scope = scope.define_type(builder, "%StringType");
            type_scope.define_function(
                builder,
                "concat",
                &["string[] stringsToConcatenate"],
                Some("string memory"),
            );
            let mut type_scope = scope.define_type(builder, "%TransactionType");
            type_scope.define_field(builder, "gasprice", "uint256");
            type_scope.define_field(builder, "origin", "address");
            let mut type_scope = scope.define_type(builder, "%ContractTypeType");
            type_scope.define_field(builder, "name", "string");
            type_scope.define_field(builder, "creationCode", "bytes");
            type_scope.define_field(builder, "runtimeCode", "bytes");
            type_scope.define_field(builder, "interfaceId", "bytes4");
            let mut type_scope = scope.define_type(builder, "%InterfaceTypeType");
            type_scope.define_field(builder, "name", "string");
            type_scope.define_field(builder, "interfaceId", "bytes4");
            let mut type_scope = scope.define_type(builder, "%IntTypeType");
            type_scope.define_field(builder, "min", "int256");
            type_scope.define_field(builder, "max", "int256");
            scope.define_field(builder, "%_", "%Function");
            scope.define_field(builder, "abi", "%AbiType");
            scope.define_field(builder, "block", "%BlockType");
            scope.define_field(builder, "bytes", "%BytesType");
            scope.define_field(builder, "msg", "%MessageType");
            scope.define_field(builder, "string", "%StringType");
            scope.define_field(builder, "tx", "%TransactionType");
        }
        // YulBuiltIns
        {
            let mut scope = scope.new_context(builder, "@yul");
            let mut type_scope = scope.define_type(builder, "%YulExternal");
            type_scope.define_field(builder, "slot", "uint256");
            type_scope.define_field(builder, "offset", "uint256");
            type_scope.define_field(builder, "length", "uint256");
            scope.define_function(builder, "add", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "addmod",
                &["uint256 x", "uint256 y", "uint256 m"],
                Some("uint256"),
            );
            scope.define_function(builder, "address", &[], Some("uint256"));
            scope.define_function(builder, "and", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "balance", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "blockhash", &["uint256 b"], Some("uint256"));
            scope.define_function(
                builder,
                "byte",
                &["uint256 n", "uint256 x"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "callcode",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 v",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "calldatacopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "calldataload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "calldatasize", &[], Some("uint256"));
            scope.define_function(builder, "caller", &[], Some("uint256"));
            scope.define_function(
                builder,
                "call",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 v",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(builder, "callvalue", &[], Some("uint256"));
            scope.define_function(
                builder,
                "codecopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "codesize", &[], Some("uint256"));
            scope.define_function(builder, "coinbase", &[], Some("uint256"));
            scope.define_function(
                builder,
                "create",
                &["uint256 v", "uint256 p", "uint256 n"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "delegatecall",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(builder, "div", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "eq", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "exp", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "extcodecopy",
                &["uint256 a", "uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "extcodesize", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "gas", &[], Some("uint256"));
            scope.define_function(builder, "gaslimit", &[], Some("uint256"));
            scope.define_function(builder, "gasprice", &[], Some("uint256"));
            scope.define_function(builder, "gt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "invalid", &[], None);
            scope.define_function(builder, "iszero", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "log0", &["uint256 p", "uint256 s"], None);
            scope.define_function(
                builder,
                "log1",
                &["uint256 p", "uint256 s", "uint256 t1"],
                None,
            );
            scope.define_function(
                builder,
                "log2",
                &["uint256 p", "uint256 s", "uint256 t1", "uint256 t2"],
                None,
            );
            scope.define_function(
                builder,
                "log3",
                &[
                    "uint256 p",
                    "uint256 s",
                    "uint256 t1",
                    "uint256 t2",
                    "uint256 t3",
                ],
                None,
            );
            scope.define_function(
                builder,
                "log4",
                &[
                    "uint256 p",
                    "uint256 s",
                    "uint256 t1",
                    "uint256 t2",
                    "uint256 t3",
                ],
                None,
            );
            scope.define_function(builder, "lt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "mload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "mod", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "msize", &[], Some("uint256"));
            scope.define_function(builder, "mstore8", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "mstore", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "mul", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "mulmod",
                &["uint256 x", "uint256 y", "uint256 m"],
                Some("uint256"),
            );
            scope.define_function(builder, "not", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "number", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "origin", &[], Some("uint256"));
            scope.define_function(builder, "or", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "pop", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "return", &["uint256 p", "uint256 s"], None);
            scope.define_function(builder, "revert", &["uint256 p", "uint256 s"], None);
            scope.define_function(
                builder,
                "sdiv",
                &["uint256 x", "uint256 y"],
                Some("uint256"),
            );
            scope.define_function(builder, "selfdestruct", &["uint256 a"], None);
            scope.define_function(builder, "sgt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "signextend",
                &["uint256 i", "uint256 x"],
                Some("uint256"),
            );
            scope.define_function(builder, "sload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "slt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "smod",
                &["uint256 x", "uint256 y"],
                Some("uint256"),
            );
            scope.define_function(builder, "sstore", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "stop", &[], None);
            scope.define_function(builder, "sub", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "timestamp", &[], Some("uint256"));
            scope.define_function(builder, "xor", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "keccak256",
                &["uint256 p", "uint256 n"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "returndatacopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "returndatasize", &[], Some("uint256"));
            scope.define_function(
                builder,
                "staticcall",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "create2",
                &["uint256 v", "uint256 p", "uint256 n", "uint256 s"],
                Some("uint256"),
            );
            scope.define_function(builder, "extcodehash", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "sar", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "shl", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "shr", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "chainid", &[], Some("uint256"));
            scope.define_function(builder, "selfbalance", &[], Some("uint256"));
            scope.define_function(builder, "difficulty", &[], Some("uint256"));
        }
    } else if *version < Version::new(0, 8, 4) {
        // 0.8.2
        // SolidityBuiltIns
        {
            scope.define_function(
                builder,
                "addmod",
                &["uint x", "uint y", "uint k"],
                Some("uint256"),
            );
            scope.define_function(builder, "assert", &["bool condition"], None);
            scope.define_function(builder, "blockhash", &["uint blockNumber"], Some("bytes32"));
            scope.define_function(
                builder,
                "ecrecover",
                &["bytes32 hash", "uint8 v", "bytes32 r", "bytes32 s"],
                Some("address"),
            );
            scope.define_function(builder, "gasleft", &[], Some("uint256"));
            scope.define_function(builder, "keccak256", &["bytes memory"], Some("bytes32"));
            scope.define_function(
                builder,
                "mulmod",
                &["uint x", "uint y", "uint k"],
                Some("uint256"),
            );
            scope.define_function(builder, "require", &["bool condition"], None);
            scope.define_function(
                builder,
                "require",
                &["bool condition", "string memory message"],
                None,
            );
            scope.define_function(builder, "revert", &[], None);
            scope.define_function(builder, "revert", &["string memory reason"], None);
            scope.define_function(builder, "ripemd160", &["bytes memory"], Some("bytes20"));
            scope.define_function(
                builder,
                "selfdestruct",
                &["address payable recipient"],
                None,
            );
            scope.define_function(builder, "sha256", &["bytes memory"], Some("bytes32"));
            let mut type_scope = scope.define_type(builder, "%AbiType");
            type_scope.define_function(
                builder,
                "decode",
                &["bytes memory encodedData", "%Type[] encodedTypesTuple"],
                Some("%Any[]"),
            );
            type_scope.define_function(
                builder,
                "encode",
                &["%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodePacked",
                &["%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodeWithSelector",
                &["bytes4 selector", "%Any[] functionArgumentsTuple"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodeWithSignature",
                &["string memory signature", "%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "address");
            type_scope.define_field(builder, "balance", "uint256");
            type_scope.define_field(builder, "code", "bytes");
            type_scope.define_field(builder, "codehash", "bytes32");
            type_scope.define_function(
                builder,
                "call",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(
                builder,
                "delegatecall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "send", &["uint256 amount"], Some("bool"));
            type_scope.define_function(
                builder,
                "staticcall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "address payable");
            type_scope.define_field(builder, "balance", "uint256");
            type_scope.define_field(builder, "code", "bytes");
            type_scope.define_field(builder, "codehash", "bytes32");
            type_scope.define_function(
                builder,
                "call",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(
                builder,
                "delegatecall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "send", &["uint256 amount"], Some("bool"));
            type_scope.define_function(
                builder,
                "staticcall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "transfer", &["uint256 amount"], None);
            let mut type_scope = scope.define_type(builder, "%Array");
            type_scope.define_field(builder, "length", "uint256");
            type_scope.define_function(builder, "push", &[], Some("%ValueType"));
            type_scope.define_function(builder, "push", &["%ValueType element"], None);
            type_scope.define_function(builder, "pop", &[], None);
            let mut type_scope = scope.define_type(builder, "%FixedArray");
            type_scope.define_field(builder, "length", "uint256");
            let mut type_scope = scope.define_type(builder, "%BlockType");
            type_scope.define_field(builder, "chainid", "uint256");
            type_scope.define_field(builder, "coinbase", "address payable");
            type_scope.define_field(builder, "difficulty", "uint256");
            type_scope.define_field(builder, "gaslimit", "uint256");
            type_scope.define_field(builder, "number", "uint256");
            type_scope.define_field(builder, "timestamp", "uint256");
            let mut type_scope = scope.define_type(builder, "bytes");
            type_scope.define_field(builder, "length", "uint256");
            let mut type_scope = scope.define_type(builder, "%BytesType");
            type_scope.define_function(
                builder,
                "concat",
                &["bytes[] bytesToConcatenate"],
                Some("bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "%CallOptions");
            type_scope.define_field(builder, "gas", "uint256");
            type_scope.define_field(builder, "salt", "uint256");
            type_scope.define_field(builder, "value", "uint256");
            let mut type_scope = scope.define_type(builder, "Error");
            type_scope.define_field(builder, "reason", "string");
            _ = scope.define_type(builder, "%Function");
            let mut type_scope = scope.define_type(builder, "%ExternalFunction");
            type_scope.define_field(builder, "address", "address");
            type_scope.define_field(builder, "selector", "bytes4");
            let mut type_scope = scope.define_type(builder, "%MessageType");
            type_scope.define_field(builder, "data", "bytes");
            type_scope.define_field(builder, "sender", "address");
            type_scope.define_field(builder, "sig", "bytes4");
            type_scope.define_field(builder, "value", "uint256");
            let mut type_scope = scope.define_type(builder, "Panic");
            type_scope.define_field(builder, "errorCode", "uint256");
            let mut type_scope = scope.define_type(builder, "%StringType");
            type_scope.define_function(
                builder,
                "concat",
                &["string[] stringsToConcatenate"],
                Some("string memory"),
            );
            let mut type_scope = scope.define_type(builder, "%TransactionType");
            type_scope.define_field(builder, "gasprice", "uint256");
            type_scope.define_field(builder, "origin", "address");
            let mut type_scope = scope.define_type(builder, "%ContractTypeType");
            type_scope.define_field(builder, "name", "string");
            type_scope.define_field(builder, "creationCode", "bytes");
            type_scope.define_field(builder, "runtimeCode", "bytes");
            type_scope.define_field(builder, "interfaceId", "bytes4");
            let mut type_scope = scope.define_type(builder, "%InterfaceTypeType");
            type_scope.define_field(builder, "name", "string");
            type_scope.define_field(builder, "interfaceId", "bytes4");
            let mut type_scope = scope.define_type(builder, "%IntTypeType");
            type_scope.define_field(builder, "min", "int256");
            type_scope.define_field(builder, "max", "int256");
            scope.define_field(builder, "%_", "%Function");
            scope.define_field(builder, "abi", "%AbiType");
            scope.define_field(builder, "block", "%BlockType");
            scope.define_field(builder, "bytes", "%BytesType");
            scope.define_field(builder, "msg", "%MessageType");
            scope.define_field(builder, "string", "%StringType");
            scope.define_field(builder, "tx", "%TransactionType");
        }
        // YulBuiltIns
        {
            let mut scope = scope.new_context(builder, "@yul");
            let mut type_scope = scope.define_type(builder, "%YulExternal");
            type_scope.define_field(builder, "slot", "uint256");
            type_scope.define_field(builder, "offset", "uint256");
            type_scope.define_field(builder, "length", "uint256");
            scope.define_function(builder, "add", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "addmod",
                &["uint256 x", "uint256 y", "uint256 m"],
                Some("uint256"),
            );
            scope.define_function(builder, "address", &[], Some("uint256"));
            scope.define_function(builder, "and", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "balance", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "blockhash", &["uint256 b"], Some("uint256"));
            scope.define_function(
                builder,
                "byte",
                &["uint256 n", "uint256 x"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "callcode",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 v",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "calldatacopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "calldataload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "calldatasize", &[], Some("uint256"));
            scope.define_function(builder, "caller", &[], Some("uint256"));
            scope.define_function(
                builder,
                "call",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 v",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(builder, "callvalue", &[], Some("uint256"));
            scope.define_function(
                builder,
                "codecopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "codesize", &[], Some("uint256"));
            scope.define_function(builder, "coinbase", &[], Some("uint256"));
            scope.define_function(
                builder,
                "create",
                &["uint256 v", "uint256 p", "uint256 n"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "delegatecall",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(builder, "div", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "eq", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "exp", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "extcodecopy",
                &["uint256 a", "uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "extcodesize", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "gas", &[], Some("uint256"));
            scope.define_function(builder, "gaslimit", &[], Some("uint256"));
            scope.define_function(builder, "gasprice", &[], Some("uint256"));
            scope.define_function(builder, "gt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "invalid", &[], None);
            scope.define_function(builder, "iszero", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "log0", &["uint256 p", "uint256 s"], None);
            scope.define_function(
                builder,
                "log1",
                &["uint256 p", "uint256 s", "uint256 t1"],
                None,
            );
            scope.define_function(
                builder,
                "log2",
                &["uint256 p", "uint256 s", "uint256 t1", "uint256 t2"],
                None,
            );
            scope.define_function(
                builder,
                "log3",
                &[
                    "uint256 p",
                    "uint256 s",
                    "uint256 t1",
                    "uint256 t2",
                    "uint256 t3",
                ],
                None,
            );
            scope.define_function(
                builder,
                "log4",
                &[
                    "uint256 p",
                    "uint256 s",
                    "uint256 t1",
                    "uint256 t2",
                    "uint256 t3",
                ],
                None,
            );
            scope.define_function(builder, "lt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "mload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "mod", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "msize", &[], Some("uint256"));
            scope.define_function(builder, "mstore8", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "mstore", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "mul", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "mulmod",
                &["uint256 x", "uint256 y", "uint256 m"],
                Some("uint256"),
            );
            scope.define_function(builder, "not", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "number", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "origin", &[], Some("uint256"));
            scope.define_function(builder, "or", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "pop", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "return", &["uint256 p", "uint256 s"], None);
            scope.define_function(builder, "revert", &["uint256 p", "uint256 s"], None);
            scope.define_function(
                builder,
                "sdiv",
                &["uint256 x", "uint256 y"],
                Some("uint256"),
            );
            scope.define_function(builder, "selfdestruct", &["uint256 a"], None);
            scope.define_function(builder, "sgt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "signextend",
                &["uint256 i", "uint256 x"],
                Some("uint256"),
            );
            scope.define_function(builder, "sload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "slt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "smod",
                &["uint256 x", "uint256 y"],
                Some("uint256"),
            );
            scope.define_function(builder, "sstore", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "stop", &[], None);
            scope.define_function(builder, "sub", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "timestamp", &[], Some("uint256"));
            scope.define_function(builder, "xor", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "keccak256",
                &["uint256 p", "uint256 n"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "returndatacopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "returndatasize", &[], Some("uint256"));
            scope.define_function(
                builder,
                "staticcall",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "create2",
                &["uint256 v", "uint256 p", "uint256 n", "uint256 s"],
                Some("uint256"),
            );
            scope.define_function(builder, "extcodehash", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "sar", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "shl", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "shr", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "chainid", &[], Some("uint256"));
            scope.define_function(builder, "selfbalance", &[], Some("uint256"));
            scope.define_function(builder, "difficulty", &[], Some("uint256"));
        }
    } else if *version < Version::new(0, 8, 7) {
        // 0.8.4
        // SolidityBuiltIns
        {
            scope.define_function(
                builder,
                "addmod",
                &["uint x", "uint y", "uint k"],
                Some("uint256"),
            );
            scope.define_function(builder, "assert", &["bool condition"], None);
            scope.define_function(builder, "blockhash", &["uint blockNumber"], Some("bytes32"));
            scope.define_function(
                builder,
                "ecrecover",
                &["bytes32 hash", "uint8 v", "bytes32 r", "bytes32 s"],
                Some("address"),
            );
            scope.define_function(builder, "gasleft", &[], Some("uint256"));
            scope.define_function(builder, "keccak256", &["bytes memory"], Some("bytes32"));
            scope.define_function(
                builder,
                "mulmod",
                &["uint x", "uint y", "uint k"],
                Some("uint256"),
            );
            scope.define_function(builder, "require", &["bool condition"], None);
            scope.define_function(
                builder,
                "require",
                &["bool condition", "string memory message"],
                None,
            );
            scope.define_function(builder, "revert", &[], None);
            scope.define_function(builder, "revert", &["string memory reason"], None);
            scope.define_function(builder, "ripemd160", &["bytes memory"], Some("bytes20"));
            scope.define_function(
                builder,
                "selfdestruct",
                &["address payable recipient"],
                None,
            );
            scope.define_function(builder, "sha256", &["bytes memory"], Some("bytes32"));
            let mut type_scope = scope.define_type(builder, "%AbiType");
            type_scope.define_function(
                builder,
                "decode",
                &["bytes memory encodedData", "%Type[] encodedTypesTuple"],
                Some("%Any[]"),
            );
            type_scope.define_function(
                builder,
                "encode",
                &["%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodePacked",
                &["%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodeWithSelector",
                &["bytes4 selector", "%Any[] functionArgumentsTuple"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodeWithSignature",
                &["string memory signature", "%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "address");
            type_scope.define_field(builder, "balance", "uint256");
            type_scope.define_field(builder, "code", "bytes");
            type_scope.define_field(builder, "codehash", "bytes32");
            type_scope.define_function(
                builder,
                "call",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(
                builder,
                "delegatecall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "send", &["uint256 amount"], Some("bool"));
            type_scope.define_function(
                builder,
                "staticcall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "address payable");
            type_scope.define_field(builder, "balance", "uint256");
            type_scope.define_field(builder, "code", "bytes");
            type_scope.define_field(builder, "codehash", "bytes32");
            type_scope.define_function(
                builder,
                "call",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(
                builder,
                "delegatecall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "send", &["uint256 amount"], Some("bool"));
            type_scope.define_function(
                builder,
                "staticcall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "transfer", &["uint256 amount"], None);
            let mut type_scope = scope.define_type(builder, "%Array");
            type_scope.define_field(builder, "length", "uint256");
            type_scope.define_function(builder, "push", &[], Some("%ValueType"));
            type_scope.define_function(builder, "push", &["%ValueType element"], None);
            type_scope.define_function(builder, "pop", &[], None);
            let mut type_scope = scope.define_type(builder, "%FixedArray");
            type_scope.define_field(builder, "length", "uint256");
            let mut type_scope = scope.define_type(builder, "%BlockType");
            type_scope.define_field(builder, "chainid", "uint256");
            type_scope.define_field(builder, "coinbase", "address payable");
            type_scope.define_field(builder, "difficulty", "uint256");
            type_scope.define_field(builder, "gaslimit", "uint256");
            type_scope.define_field(builder, "number", "uint256");
            type_scope.define_field(builder, "timestamp", "uint256");
            let mut type_scope = scope.define_type(builder, "bytes");
            type_scope.define_field(builder, "length", "uint256");
            let mut type_scope = scope.define_type(builder, "%BytesType");
            type_scope.define_function(
                builder,
                "concat",
                &["bytes[] bytesToConcatenate"],
                Some("bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "%CallOptions");
            type_scope.define_field(builder, "gas", "uint256");
            type_scope.define_field(builder, "salt", "uint256");
            type_scope.define_field(builder, "value", "uint256");
            let mut type_scope = scope.define_type(builder, "Error");
            type_scope.define_field(builder, "reason", "string");
            let mut type_scope = scope.define_type(builder, "%ErrorType");
            type_scope.define_field(builder, "selector", "bytes4");
            _ = scope.define_type(builder, "%Function");
            let mut type_scope = scope.define_type(builder, "%ExternalFunction");
            type_scope.define_field(builder, "address", "address");
            type_scope.define_field(builder, "selector", "bytes4");
            let mut type_scope = scope.define_type(builder, "%MessageType");
            type_scope.define_field(builder, "data", "bytes");
            type_scope.define_field(builder, "sender", "address");
            type_scope.define_field(builder, "sig", "bytes4");
            type_scope.define_field(builder, "value", "uint256");
            let mut type_scope = scope.define_type(builder, "Panic");
            type_scope.define_field(builder, "errorCode", "uint256");
            let mut type_scope = scope.define_type(builder, "%StringType");
            type_scope.define_function(
                builder,
                "concat",
                &["string[] stringsToConcatenate"],
                Some("string memory"),
            );
            let mut type_scope = scope.define_type(builder, "%TransactionType");
            type_scope.define_field(builder, "gasprice", "uint256");
            type_scope.define_field(builder, "origin", "address");
            let mut type_scope = scope.define_type(builder, "%ContractTypeType");
            type_scope.define_field(builder, "name", "string");
            type_scope.define_field(builder, "creationCode", "bytes");
            type_scope.define_field(builder, "runtimeCode", "bytes");
            type_scope.define_field(builder, "interfaceId", "bytes4");
            let mut type_scope = scope.define_type(builder, "%InterfaceTypeType");
            type_scope.define_field(builder, "name", "string");
            type_scope.define_field(builder, "interfaceId", "bytes4");
            let mut type_scope = scope.define_type(builder, "%IntTypeType");
            type_scope.define_field(builder, "min", "int256");
            type_scope.define_field(builder, "max", "int256");
            scope.define_field(builder, "%_", "%Function");
            scope.define_field(builder, "abi", "%AbiType");
            scope.define_field(builder, "block", "%BlockType");
            scope.define_field(builder, "bytes", "%BytesType");
            scope.define_field(builder, "msg", "%MessageType");
            scope.define_field(builder, "string", "%StringType");
            scope.define_field(builder, "tx", "%TransactionType");
        }
        // YulBuiltIns
        {
            let mut scope = scope.new_context(builder, "@yul");
            let mut type_scope = scope.define_type(builder, "%YulExternal");
            type_scope.define_field(builder, "slot", "uint256");
            type_scope.define_field(builder, "offset", "uint256");
            type_scope.define_field(builder, "length", "uint256");
            scope.define_function(builder, "add", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "addmod",
                &["uint256 x", "uint256 y", "uint256 m"],
                Some("uint256"),
            );
            scope.define_function(builder, "address", &[], Some("uint256"));
            scope.define_function(builder, "and", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "balance", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "blockhash", &["uint256 b"], Some("uint256"));
            scope.define_function(
                builder,
                "byte",
                &["uint256 n", "uint256 x"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "callcode",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 v",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "calldatacopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "calldataload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "calldatasize", &[], Some("uint256"));
            scope.define_function(builder, "caller", &[], Some("uint256"));
            scope.define_function(
                builder,
                "call",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 v",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(builder, "callvalue", &[], Some("uint256"));
            scope.define_function(
                builder,
                "codecopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "codesize", &[], Some("uint256"));
            scope.define_function(builder, "coinbase", &[], Some("uint256"));
            scope.define_function(
                builder,
                "create",
                &["uint256 v", "uint256 p", "uint256 n"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "delegatecall",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(builder, "div", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "eq", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "exp", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "extcodecopy",
                &["uint256 a", "uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "extcodesize", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "gas", &[], Some("uint256"));
            scope.define_function(builder, "gaslimit", &[], Some("uint256"));
            scope.define_function(builder, "gasprice", &[], Some("uint256"));
            scope.define_function(builder, "gt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "invalid", &[], None);
            scope.define_function(builder, "iszero", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "log0", &["uint256 p", "uint256 s"], None);
            scope.define_function(
                builder,
                "log1",
                &["uint256 p", "uint256 s", "uint256 t1"],
                None,
            );
            scope.define_function(
                builder,
                "log2",
                &["uint256 p", "uint256 s", "uint256 t1", "uint256 t2"],
                None,
            );
            scope.define_function(
                builder,
                "log3",
                &[
                    "uint256 p",
                    "uint256 s",
                    "uint256 t1",
                    "uint256 t2",
                    "uint256 t3",
                ],
                None,
            );
            scope.define_function(
                builder,
                "log4",
                &[
                    "uint256 p",
                    "uint256 s",
                    "uint256 t1",
                    "uint256 t2",
                    "uint256 t3",
                ],
                None,
            );
            scope.define_function(builder, "lt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "mload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "mod", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "msize", &[], Some("uint256"));
            scope.define_function(builder, "mstore8", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "mstore", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "mul", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "mulmod",
                &["uint256 x", "uint256 y", "uint256 m"],
                Some("uint256"),
            );
            scope.define_function(builder, "not", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "number", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "origin", &[], Some("uint256"));
            scope.define_function(builder, "or", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "pop", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "return", &["uint256 p", "uint256 s"], None);
            scope.define_function(builder, "revert", &["uint256 p", "uint256 s"], None);
            scope.define_function(
                builder,
                "sdiv",
                &["uint256 x", "uint256 y"],
                Some("uint256"),
            );
            scope.define_function(builder, "selfdestruct", &["uint256 a"], None);
            scope.define_function(builder, "sgt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "signextend",
                &["uint256 i", "uint256 x"],
                Some("uint256"),
            );
            scope.define_function(builder, "sload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "slt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "smod",
                &["uint256 x", "uint256 y"],
                Some("uint256"),
            );
            scope.define_function(builder, "sstore", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "stop", &[], None);
            scope.define_function(builder, "sub", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "timestamp", &[], Some("uint256"));
            scope.define_function(builder, "xor", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "keccak256",
                &["uint256 p", "uint256 n"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "returndatacopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "returndatasize", &[], Some("uint256"));
            scope.define_function(
                builder,
                "staticcall",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "create2",
                &["uint256 v", "uint256 p", "uint256 n", "uint256 s"],
                Some("uint256"),
            );
            scope.define_function(builder, "extcodehash", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "sar", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "shl", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "shr", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "chainid", &[], Some("uint256"));
            scope.define_function(builder, "selfbalance", &[], Some("uint256"));
            scope.define_function(builder, "difficulty", &[], Some("uint256"));
        }
    } else if *version < Version::new(0, 8, 8) {
        // 0.8.7
        // SolidityBuiltIns
        {
            scope.define_function(
                builder,
                "addmod",
                &["uint x", "uint y", "uint k"],
                Some("uint256"),
            );
            scope.define_function(builder, "assert", &["bool condition"], None);
            scope.define_function(builder, "blockhash", &["uint blockNumber"], Some("bytes32"));
            scope.define_function(
                builder,
                "ecrecover",
                &["bytes32 hash", "uint8 v", "bytes32 r", "bytes32 s"],
                Some("address"),
            );
            scope.define_function(builder, "gasleft", &[], Some("uint256"));
            scope.define_function(builder, "keccak256", &["bytes memory"], Some("bytes32"));
            scope.define_function(
                builder,
                "mulmod",
                &["uint x", "uint y", "uint k"],
                Some("uint256"),
            );
            scope.define_function(builder, "require", &["bool condition"], None);
            scope.define_function(
                builder,
                "require",
                &["bool condition", "string memory message"],
                None,
            );
            scope.define_function(builder, "revert", &[], None);
            scope.define_function(builder, "revert", &["string memory reason"], None);
            scope.define_function(builder, "ripemd160", &["bytes memory"], Some("bytes20"));
            scope.define_function(
                builder,
                "selfdestruct",
                &["address payable recipient"],
                None,
            );
            scope.define_function(builder, "sha256", &["bytes memory"], Some("bytes32"));
            let mut type_scope = scope.define_type(builder, "%AbiType");
            type_scope.define_function(
                builder,
                "decode",
                &["bytes memory encodedData", "%Type[] encodedTypesTuple"],
                Some("%Any[]"),
            );
            type_scope.define_function(
                builder,
                "encode",
                &["%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodePacked",
                &["%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodeWithSelector",
                &["bytes4 selector", "%Any[] functionArgumentsTuple"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodeWithSignature",
                &["string memory signature", "%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "address");
            type_scope.define_field(builder, "balance", "uint256");
            type_scope.define_field(builder, "code", "bytes");
            type_scope.define_field(builder, "codehash", "bytes32");
            type_scope.define_function(
                builder,
                "call",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(
                builder,
                "delegatecall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "send", &["uint256 amount"], Some("bool"));
            type_scope.define_function(
                builder,
                "staticcall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "address payable");
            type_scope.define_field(builder, "balance", "uint256");
            type_scope.define_field(builder, "code", "bytes");
            type_scope.define_field(builder, "codehash", "bytes32");
            type_scope.define_function(
                builder,
                "call",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(
                builder,
                "delegatecall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "send", &["uint256 amount"], Some("bool"));
            type_scope.define_function(
                builder,
                "staticcall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "transfer", &["uint256 amount"], None);
            let mut type_scope = scope.define_type(builder, "%Array");
            type_scope.define_field(builder, "length", "uint256");
            type_scope.define_function(builder, "push", &[], Some("%ValueType"));
            type_scope.define_function(builder, "push", &["%ValueType element"], None);
            type_scope.define_function(builder, "pop", &[], None);
            let mut type_scope = scope.define_type(builder, "%FixedArray");
            type_scope.define_field(builder, "length", "uint256");
            let mut type_scope = scope.define_type(builder, "%BlockType");
            type_scope.define_field(builder, "basefee", "uint256");
            type_scope.define_field(builder, "chainid", "uint256");
            type_scope.define_field(builder, "coinbase", "address payable");
            type_scope.define_field(builder, "difficulty", "uint256");
            type_scope.define_field(builder, "gaslimit", "uint256");
            type_scope.define_field(builder, "number", "uint256");
            type_scope.define_field(builder, "timestamp", "uint256");
            let mut type_scope = scope.define_type(builder, "bytes");
            type_scope.define_field(builder, "length", "uint256");
            let mut type_scope = scope.define_type(builder, "%BytesType");
            type_scope.define_function(
                builder,
                "concat",
                &["bytes[] bytesToConcatenate"],
                Some("bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "%CallOptions");
            type_scope.define_field(builder, "gas", "uint256");
            type_scope.define_field(builder, "salt", "uint256");
            type_scope.define_field(builder, "value", "uint256");
            let mut type_scope = scope.define_type(builder, "Error");
            type_scope.define_field(builder, "reason", "string");
            let mut type_scope = scope.define_type(builder, "%ErrorType");
            type_scope.define_field(builder, "selector", "bytes4");
            _ = scope.define_type(builder, "%Function");
            let mut type_scope = scope.define_type(builder, "%ExternalFunction");
            type_scope.define_field(builder, "address", "address");
            type_scope.define_field(builder, "selector", "bytes4");
            let mut type_scope = scope.define_type(builder, "%MessageType");
            type_scope.define_field(builder, "data", "bytes");
            type_scope.define_field(builder, "sender", "address");
            type_scope.define_field(builder, "sig", "bytes4");
            type_scope.define_field(builder, "value", "uint256");
            let mut type_scope = scope.define_type(builder, "Panic");
            type_scope.define_field(builder, "errorCode", "uint256");
            let mut type_scope = scope.define_type(builder, "%StringType");
            type_scope.define_function(
                builder,
                "concat",
                &["string[] stringsToConcatenate"],
                Some("string memory"),
            );
            let mut type_scope = scope.define_type(builder, "%TransactionType");
            type_scope.define_field(builder, "gasprice", "uint256");
            type_scope.define_field(builder, "origin", "address");
            let mut type_scope = scope.define_type(builder, "%ContractTypeType");
            type_scope.define_field(builder, "name", "string");
            type_scope.define_field(builder, "creationCode", "bytes");
            type_scope.define_field(builder, "runtimeCode", "bytes");
            type_scope.define_field(builder, "interfaceId", "bytes4");
            let mut type_scope = scope.define_type(builder, "%InterfaceTypeType");
            type_scope.define_field(builder, "name", "string");
            type_scope.define_field(builder, "interfaceId", "bytes4");
            let mut type_scope = scope.define_type(builder, "%IntTypeType");
            type_scope.define_field(builder, "min", "int256");
            type_scope.define_field(builder, "max", "int256");
            scope.define_field(builder, "%_", "%Function");
            scope.define_field(builder, "abi", "%AbiType");
            scope.define_field(builder, "block", "%BlockType");
            scope.define_field(builder, "bytes", "%BytesType");
            scope.define_field(builder, "msg", "%MessageType");
            scope.define_field(builder, "string", "%StringType");
            scope.define_field(builder, "tx", "%TransactionType");
        }
        // YulBuiltIns
        {
            let mut scope = scope.new_context(builder, "@yul");
            let mut type_scope = scope.define_type(builder, "%YulExternal");
            type_scope.define_field(builder, "slot", "uint256");
            type_scope.define_field(builder, "offset", "uint256");
            type_scope.define_field(builder, "length", "uint256");
            scope.define_function(builder, "add", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "addmod",
                &["uint256 x", "uint256 y", "uint256 m"],
                Some("uint256"),
            );
            scope.define_function(builder, "address", &[], Some("uint256"));
            scope.define_function(builder, "and", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "balance", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "blockhash", &["uint256 b"], Some("uint256"));
            scope.define_function(
                builder,
                "byte",
                &["uint256 n", "uint256 x"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "callcode",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 v",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "calldatacopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "calldataload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "calldatasize", &[], Some("uint256"));
            scope.define_function(builder, "caller", &[], Some("uint256"));
            scope.define_function(
                builder,
                "call",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 v",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(builder, "callvalue", &[], Some("uint256"));
            scope.define_function(
                builder,
                "codecopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "codesize", &[], Some("uint256"));
            scope.define_function(builder, "coinbase", &[], Some("uint256"));
            scope.define_function(
                builder,
                "create",
                &["uint256 v", "uint256 p", "uint256 n"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "delegatecall",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(builder, "div", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "eq", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "exp", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "extcodecopy",
                &["uint256 a", "uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "extcodesize", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "gas", &[], Some("uint256"));
            scope.define_function(builder, "gaslimit", &[], Some("uint256"));
            scope.define_function(builder, "gasprice", &[], Some("uint256"));
            scope.define_function(builder, "gt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "invalid", &[], None);
            scope.define_function(builder, "iszero", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "log0", &["uint256 p", "uint256 s"], None);
            scope.define_function(
                builder,
                "log1",
                &["uint256 p", "uint256 s", "uint256 t1"],
                None,
            );
            scope.define_function(
                builder,
                "log2",
                &["uint256 p", "uint256 s", "uint256 t1", "uint256 t2"],
                None,
            );
            scope.define_function(
                builder,
                "log3",
                &[
                    "uint256 p",
                    "uint256 s",
                    "uint256 t1",
                    "uint256 t2",
                    "uint256 t3",
                ],
                None,
            );
            scope.define_function(
                builder,
                "log4",
                &[
                    "uint256 p",
                    "uint256 s",
                    "uint256 t1",
                    "uint256 t2",
                    "uint256 t3",
                ],
                None,
            );
            scope.define_function(builder, "lt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "mload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "mod", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "msize", &[], Some("uint256"));
            scope.define_function(builder, "mstore8", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "mstore", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "mul", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "mulmod",
                &["uint256 x", "uint256 y", "uint256 m"],
                Some("uint256"),
            );
            scope.define_function(builder, "not", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "number", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "origin", &[], Some("uint256"));
            scope.define_function(builder, "or", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "pop", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "return", &["uint256 p", "uint256 s"], None);
            scope.define_function(builder, "revert", &["uint256 p", "uint256 s"], None);
            scope.define_function(
                builder,
                "sdiv",
                &["uint256 x", "uint256 y"],
                Some("uint256"),
            );
            scope.define_function(builder, "selfdestruct", &["uint256 a"], None);
            scope.define_function(builder, "sgt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "signextend",
                &["uint256 i", "uint256 x"],
                Some("uint256"),
            );
            scope.define_function(builder, "sload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "slt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "smod",
                &["uint256 x", "uint256 y"],
                Some("uint256"),
            );
            scope.define_function(builder, "sstore", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "stop", &[], None);
            scope.define_function(builder, "sub", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "timestamp", &[], Some("uint256"));
            scope.define_function(builder, "xor", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "keccak256",
                &["uint256 p", "uint256 n"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "returndatacopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "returndatasize", &[], Some("uint256"));
            scope.define_function(
                builder,
                "staticcall",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "create2",
                &["uint256 v", "uint256 p", "uint256 n", "uint256 s"],
                Some("uint256"),
            );
            scope.define_function(builder, "extcodehash", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "sar", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "shl", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "shr", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "chainid", &[], Some("uint256"));
            scope.define_function(builder, "selfbalance", &[], Some("uint256"));
            scope.define_function(builder, "basefee", &[], Some("uint256"));
            scope.define_function(builder, "difficulty", &[], Some("uint256"));
        }
    } else if *version < Version::new(0, 8, 11) {
        // 0.8.8
        // SolidityBuiltIns
        {
            scope.define_function(
                builder,
                "addmod",
                &["uint x", "uint y", "uint k"],
                Some("uint256"),
            );
            scope.define_function(builder, "assert", &["bool condition"], None);
            scope.define_function(builder, "blockhash", &["uint blockNumber"], Some("bytes32"));
            scope.define_function(
                builder,
                "ecrecover",
                &["bytes32 hash", "uint8 v", "bytes32 r", "bytes32 s"],
                Some("address"),
            );
            scope.define_function(builder, "gasleft", &[], Some("uint256"));
            scope.define_function(builder, "keccak256", &["bytes memory"], Some("bytes32"));
            scope.define_function(
                builder,
                "mulmod",
                &["uint x", "uint y", "uint k"],
                Some("uint256"),
            );
            scope.define_function(builder, "require", &["bool condition"], None);
            scope.define_function(
                builder,
                "require",
                &["bool condition", "string memory message"],
                None,
            );
            scope.define_function(builder, "revert", &[], None);
            scope.define_function(builder, "revert", &["string memory reason"], None);
            scope.define_function(builder, "ripemd160", &["bytes memory"], Some("bytes20"));
            scope.define_function(
                builder,
                "selfdestruct",
                &["address payable recipient"],
                None,
            );
            scope.define_function(builder, "sha256", &["bytes memory"], Some("bytes32"));
            let mut type_scope = scope.define_type(builder, "%AbiType");
            type_scope.define_function(
                builder,
                "decode",
                &["bytes memory encodedData", "%Type[] encodedTypesTuple"],
                Some("%Any[]"),
            );
            type_scope.define_function(
                builder,
                "encode",
                &["%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodePacked",
                &["%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodeWithSelector",
                &["bytes4 selector", "%Any[] functionArgumentsTuple"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodeWithSignature",
                &["string memory signature", "%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "address");
            type_scope.define_field(builder, "balance", "uint256");
            type_scope.define_field(builder, "code", "bytes");
            type_scope.define_field(builder, "codehash", "bytes32");
            type_scope.define_function(
                builder,
                "call",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(
                builder,
                "delegatecall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "send", &["uint256 amount"], Some("bool"));
            type_scope.define_function(
                builder,
                "staticcall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "address payable");
            type_scope.define_field(builder, "balance", "uint256");
            type_scope.define_field(builder, "code", "bytes");
            type_scope.define_field(builder, "codehash", "bytes32");
            type_scope.define_function(
                builder,
                "call",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(
                builder,
                "delegatecall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "send", &["uint256 amount"], Some("bool"));
            type_scope.define_function(
                builder,
                "staticcall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "transfer", &["uint256 amount"], None);
            let mut type_scope = scope.define_type(builder, "%Array");
            type_scope.define_field(builder, "length", "uint256");
            type_scope.define_function(builder, "push", &[], Some("%ValueType"));
            type_scope.define_function(builder, "push", &["%ValueType element"], None);
            type_scope.define_function(builder, "pop", &[], None);
            let mut type_scope = scope.define_type(builder, "%FixedArray");
            type_scope.define_field(builder, "length", "uint256");
            let mut type_scope = scope.define_type(builder, "%BlockType");
            type_scope.define_field(builder, "basefee", "uint256");
            type_scope.define_field(builder, "chainid", "uint256");
            type_scope.define_field(builder, "coinbase", "address payable");
            type_scope.define_field(builder, "difficulty", "uint256");
            type_scope.define_field(builder, "gaslimit", "uint256");
            type_scope.define_field(builder, "number", "uint256");
            type_scope.define_field(builder, "timestamp", "uint256");
            let mut type_scope = scope.define_type(builder, "bytes");
            type_scope.define_field(builder, "length", "uint256");
            let mut type_scope = scope.define_type(builder, "%BytesType");
            type_scope.define_function(
                builder,
                "concat",
                &["bytes[] bytesToConcatenate"],
                Some("bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "%CallOptions");
            type_scope.define_field(builder, "gas", "uint256");
            type_scope.define_field(builder, "salt", "uint256");
            type_scope.define_field(builder, "value", "uint256");
            let mut type_scope = scope.define_type(builder, "Error");
            type_scope.define_field(builder, "reason", "string");
            let mut type_scope = scope.define_type(builder, "%ErrorType");
            type_scope.define_field(builder, "selector", "bytes4");
            _ = scope.define_type(builder, "%Function");
            let mut type_scope = scope.define_type(builder, "%ExternalFunction");
            type_scope.define_field(builder, "address", "address");
            type_scope.define_field(builder, "selector", "bytes4");
            let mut type_scope = scope.define_type(builder, "%MessageType");
            type_scope.define_field(builder, "data", "bytes");
            type_scope.define_field(builder, "sender", "address");
            type_scope.define_field(builder, "sig", "bytes4");
            type_scope.define_field(builder, "value", "uint256");
            let mut type_scope = scope.define_type(builder, "Panic");
            type_scope.define_field(builder, "errorCode", "uint256");
            let mut type_scope = scope.define_type(builder, "%StringType");
            type_scope.define_function(
                builder,
                "concat",
                &["string[] stringsToConcatenate"],
                Some("string memory"),
            );
            let mut type_scope = scope.define_type(builder, "%TransactionType");
            type_scope.define_field(builder, "gasprice", "uint256");
            type_scope.define_field(builder, "origin", "address");
            let mut type_scope = scope.define_type(builder, "%ContractTypeType");
            type_scope.define_field(builder, "name", "string");
            type_scope.define_field(builder, "creationCode", "bytes");
            type_scope.define_field(builder, "runtimeCode", "bytes");
            type_scope.define_field(builder, "interfaceId", "bytes4");
            let mut type_scope = scope.define_type(builder, "%InterfaceTypeType");
            type_scope.define_field(builder, "name", "string");
            type_scope.define_field(builder, "interfaceId", "bytes4");
            let mut type_scope = scope.define_type(builder, "%IntTypeType");
            type_scope.define_field(builder, "min", "int256");
            type_scope.define_field(builder, "max", "int256");
            let mut type_scope = scope.define_type(builder, "%UserDefinedValueType");
            type_scope.define_function(
                builder,
                "wrap",
                &["%WrappedType elementaryType"],
                Some("%UserType"),
            );
            type_scope.define_function(
                builder,
                "unwrap",
                &["%UserType userType"],
                Some("%WrappedType"),
            );
            scope.define_field(builder, "%_", "%Function");
            scope.define_field(builder, "abi", "%AbiType");
            scope.define_field(builder, "block", "%BlockType");
            scope.define_field(builder, "bytes", "%BytesType");
            scope.define_field(builder, "msg", "%MessageType");
            scope.define_field(builder, "string", "%StringType");
            scope.define_field(builder, "tx", "%TransactionType");
        }
        // YulBuiltIns
        {
            let mut scope = scope.new_context(builder, "@yul");
            let mut type_scope = scope.define_type(builder, "%YulExternal");
            type_scope.define_field(builder, "slot", "uint256");
            type_scope.define_field(builder, "offset", "uint256");
            type_scope.define_field(builder, "length", "uint256");
            scope.define_function(builder, "add", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "addmod",
                &["uint256 x", "uint256 y", "uint256 m"],
                Some("uint256"),
            );
            scope.define_function(builder, "address", &[], Some("uint256"));
            scope.define_function(builder, "and", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "balance", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "blockhash", &["uint256 b"], Some("uint256"));
            scope.define_function(
                builder,
                "byte",
                &["uint256 n", "uint256 x"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "callcode",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 v",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "calldatacopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "calldataload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "calldatasize", &[], Some("uint256"));
            scope.define_function(builder, "caller", &[], Some("uint256"));
            scope.define_function(
                builder,
                "call",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 v",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(builder, "callvalue", &[], Some("uint256"));
            scope.define_function(
                builder,
                "codecopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "codesize", &[], Some("uint256"));
            scope.define_function(builder, "coinbase", &[], Some("uint256"));
            scope.define_function(
                builder,
                "create",
                &["uint256 v", "uint256 p", "uint256 n"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "delegatecall",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(builder, "div", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "eq", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "exp", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "extcodecopy",
                &["uint256 a", "uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "extcodesize", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "gas", &[], Some("uint256"));
            scope.define_function(builder, "gaslimit", &[], Some("uint256"));
            scope.define_function(builder, "gasprice", &[], Some("uint256"));
            scope.define_function(builder, "gt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "invalid", &[], None);
            scope.define_function(builder, "iszero", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "log0", &["uint256 p", "uint256 s"], None);
            scope.define_function(
                builder,
                "log1",
                &["uint256 p", "uint256 s", "uint256 t1"],
                None,
            );
            scope.define_function(
                builder,
                "log2",
                &["uint256 p", "uint256 s", "uint256 t1", "uint256 t2"],
                None,
            );
            scope.define_function(
                builder,
                "log3",
                &[
                    "uint256 p",
                    "uint256 s",
                    "uint256 t1",
                    "uint256 t2",
                    "uint256 t3",
                ],
                None,
            );
            scope.define_function(
                builder,
                "log4",
                &[
                    "uint256 p",
                    "uint256 s",
                    "uint256 t1",
                    "uint256 t2",
                    "uint256 t3",
                ],
                None,
            );
            scope.define_function(builder, "lt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "mload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "mod", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "msize", &[], Some("uint256"));
            scope.define_function(builder, "mstore8", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "mstore", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "mul", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "mulmod",
                &["uint256 x", "uint256 y", "uint256 m"],
                Some("uint256"),
            );
            scope.define_function(builder, "not", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "number", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "origin", &[], Some("uint256"));
            scope.define_function(builder, "or", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "pop", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "return", &["uint256 p", "uint256 s"], None);
            scope.define_function(builder, "revert", &["uint256 p", "uint256 s"], None);
            scope.define_function(
                builder,
                "sdiv",
                &["uint256 x", "uint256 y"],
                Some("uint256"),
            );
            scope.define_function(builder, "selfdestruct", &["uint256 a"], None);
            scope.define_function(builder, "sgt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "signextend",
                &["uint256 i", "uint256 x"],
                Some("uint256"),
            );
            scope.define_function(builder, "sload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "slt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "smod",
                &["uint256 x", "uint256 y"],
                Some("uint256"),
            );
            scope.define_function(builder, "sstore", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "stop", &[], None);
            scope.define_function(builder, "sub", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "timestamp", &[], Some("uint256"));
            scope.define_function(builder, "xor", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "keccak256",
                &["uint256 p", "uint256 n"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "returndatacopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "returndatasize", &[], Some("uint256"));
            scope.define_function(
                builder,
                "staticcall",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "create2",
                &["uint256 v", "uint256 p", "uint256 n", "uint256 s"],
                Some("uint256"),
            );
            scope.define_function(builder, "extcodehash", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "sar", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "shl", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "shr", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "chainid", &[], Some("uint256"));
            scope.define_function(builder, "selfbalance", &[], Some("uint256"));
            scope.define_function(builder, "basefee", &[], Some("uint256"));
            scope.define_function(builder, "difficulty", &[], Some("uint256"));
        }
    } else if *version < Version::new(0, 8, 18) {
        // 0.8.11
        // SolidityBuiltIns
        {
            scope.define_function(
                builder,
                "addmod",
                &["uint x", "uint y", "uint k"],
                Some("uint256"),
            );
            scope.define_function(builder, "assert", &["bool condition"], None);
            scope.define_function(builder, "blockhash", &["uint blockNumber"], Some("bytes32"));
            scope.define_function(
                builder,
                "ecrecover",
                &["bytes32 hash", "uint8 v", "bytes32 r", "bytes32 s"],
                Some("address"),
            );
            scope.define_function(builder, "gasleft", &[], Some("uint256"));
            scope.define_function(builder, "keccak256", &["bytes memory"], Some("bytes32"));
            scope.define_function(
                builder,
                "mulmod",
                &["uint x", "uint y", "uint k"],
                Some("uint256"),
            );
            scope.define_function(builder, "require", &["bool condition"], None);
            scope.define_function(
                builder,
                "require",
                &["bool condition", "string memory message"],
                None,
            );
            scope.define_function(builder, "revert", &[], None);
            scope.define_function(builder, "revert", &["string memory reason"], None);
            scope.define_function(builder, "ripemd160", &["bytes memory"], Some("bytes20"));
            scope.define_function(
                builder,
                "selfdestruct",
                &["address payable recipient"],
                None,
            );
            scope.define_function(builder, "sha256", &["bytes memory"], Some("bytes32"));
            let mut type_scope = scope.define_type(builder, "%AbiType");
            type_scope.define_function(
                builder,
                "decode",
                &["bytes memory encodedData", "%Type[] encodedTypesTuple"],
                Some("%Any[]"),
            );
            type_scope.define_function(
                builder,
                "encode",
                &["%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodeCall",
                &[
                    "function() functionPointer",
                    "%Any[] functionArgumentsTuple",
                ],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodePacked",
                &["%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodeWithSelector",
                &["bytes4 selector", "%Any[] functionArgumentsTuple"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodeWithSignature",
                &["string memory signature", "%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "address");
            type_scope.define_field(builder, "balance", "uint256");
            type_scope.define_field(builder, "code", "bytes");
            type_scope.define_field(builder, "codehash", "bytes32");
            type_scope.define_function(
                builder,
                "call",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(
                builder,
                "delegatecall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "send", &["uint256 amount"], Some("bool"));
            type_scope.define_function(
                builder,
                "staticcall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "address payable");
            type_scope.define_field(builder, "balance", "uint256");
            type_scope.define_field(builder, "code", "bytes");
            type_scope.define_field(builder, "codehash", "bytes32");
            type_scope.define_function(
                builder,
                "call",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(
                builder,
                "delegatecall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "send", &["uint256 amount"], Some("bool"));
            type_scope.define_function(
                builder,
                "staticcall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "transfer", &["uint256 amount"], None);
            let mut type_scope = scope.define_type(builder, "%Array");
            type_scope.define_field(builder, "length", "uint256");
            type_scope.define_function(builder, "push", &[], Some("%ValueType"));
            type_scope.define_function(builder, "push", &["%ValueType element"], None);
            type_scope.define_function(builder, "pop", &[], None);
            let mut type_scope = scope.define_type(builder, "%FixedArray");
            type_scope.define_field(builder, "length", "uint256");
            let mut type_scope = scope.define_type(builder, "%BlockType");
            type_scope.define_field(builder, "basefee", "uint256");
            type_scope.define_field(builder, "chainid", "uint256");
            type_scope.define_field(builder, "coinbase", "address payable");
            type_scope.define_field(builder, "difficulty", "uint256");
            type_scope.define_field(builder, "gaslimit", "uint256");
            type_scope.define_field(builder, "number", "uint256");
            type_scope.define_field(builder, "timestamp", "uint256");
            let mut type_scope = scope.define_type(builder, "bytes");
            type_scope.define_field(builder, "length", "uint256");
            let mut type_scope = scope.define_type(builder, "%BytesType");
            type_scope.define_function(
                builder,
                "concat",
                &["bytes[] bytesToConcatenate"],
                Some("bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "%CallOptions");
            type_scope.define_field(builder, "gas", "uint256");
            type_scope.define_field(builder, "salt", "uint256");
            type_scope.define_field(builder, "value", "uint256");
            let mut type_scope = scope.define_type(builder, "Error");
            type_scope.define_field(builder, "reason", "string");
            let mut type_scope = scope.define_type(builder, "%ErrorType");
            type_scope.define_field(builder, "selector", "bytes4");
            _ = scope.define_type(builder, "%Function");
            let mut type_scope = scope.define_type(builder, "%ExternalFunction");
            type_scope.define_field(builder, "address", "address");
            type_scope.define_field(builder, "selector", "bytes4");
            let mut type_scope = scope.define_type(builder, "%MessageType");
            type_scope.define_field(builder, "data", "bytes");
            type_scope.define_field(builder, "sender", "address");
            type_scope.define_field(builder, "sig", "bytes4");
            type_scope.define_field(builder, "value", "uint256");
            let mut type_scope = scope.define_type(builder, "Panic");
            type_scope.define_field(builder, "errorCode", "uint256");
            let mut type_scope = scope.define_type(builder, "%StringType");
            type_scope.define_function(
                builder,
                "concat",
                &["string[] stringsToConcatenate"],
                Some("string memory"),
            );
            let mut type_scope = scope.define_type(builder, "%TransactionType");
            type_scope.define_field(builder, "gasprice", "uint256");
            type_scope.define_field(builder, "origin", "address");
            let mut type_scope = scope.define_type(builder, "%ContractTypeType");
            type_scope.define_field(builder, "name", "string");
            type_scope.define_field(builder, "creationCode", "bytes");
            type_scope.define_field(builder, "runtimeCode", "bytes");
            type_scope.define_field(builder, "interfaceId", "bytes4");
            let mut type_scope = scope.define_type(builder, "%InterfaceTypeType");
            type_scope.define_field(builder, "name", "string");
            type_scope.define_field(builder, "interfaceId", "bytes4");
            let mut type_scope = scope.define_type(builder, "%IntTypeType");
            type_scope.define_field(builder, "min", "int256");
            type_scope.define_field(builder, "max", "int256");
            let mut type_scope = scope.define_type(builder, "%UserDefinedValueType");
            type_scope.define_function(
                builder,
                "wrap",
                &["%WrappedType elementaryType"],
                Some("%UserType"),
            );
            type_scope.define_function(
                builder,
                "unwrap",
                &["%UserType userType"],
                Some("%WrappedType"),
            );
            scope.define_field(builder, "%_", "%Function");
            scope.define_field(builder, "abi", "%AbiType");
            scope.define_field(builder, "block", "%BlockType");
            scope.define_field(builder, "bytes", "%BytesType");
            scope.define_field(builder, "msg", "%MessageType");
            scope.define_field(builder, "string", "%StringType");
            scope.define_field(builder, "tx", "%TransactionType");
        }
        // YulBuiltIns
        {
            let mut scope = scope.new_context(builder, "@yul");
            let mut type_scope = scope.define_type(builder, "%YulExternal");
            type_scope.define_field(builder, "slot", "uint256");
            type_scope.define_field(builder, "offset", "uint256");
            type_scope.define_field(builder, "length", "uint256");
            scope.define_function(builder, "add", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "addmod",
                &["uint256 x", "uint256 y", "uint256 m"],
                Some("uint256"),
            );
            scope.define_function(builder, "address", &[], Some("uint256"));
            scope.define_function(builder, "and", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "balance", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "blockhash", &["uint256 b"], Some("uint256"));
            scope.define_function(
                builder,
                "byte",
                &["uint256 n", "uint256 x"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "callcode",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 v",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "calldatacopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "calldataload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "calldatasize", &[], Some("uint256"));
            scope.define_function(builder, "caller", &[], Some("uint256"));
            scope.define_function(
                builder,
                "call",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 v",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(builder, "callvalue", &[], Some("uint256"));
            scope.define_function(
                builder,
                "codecopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "codesize", &[], Some("uint256"));
            scope.define_function(builder, "coinbase", &[], Some("uint256"));
            scope.define_function(
                builder,
                "create",
                &["uint256 v", "uint256 p", "uint256 n"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "delegatecall",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(builder, "div", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "eq", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "exp", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "extcodecopy",
                &["uint256 a", "uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "extcodesize", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "gas", &[], Some("uint256"));
            scope.define_function(builder, "gaslimit", &[], Some("uint256"));
            scope.define_function(builder, "gasprice", &[], Some("uint256"));
            scope.define_function(builder, "gt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "invalid", &[], None);
            scope.define_function(builder, "iszero", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "log0", &["uint256 p", "uint256 s"], None);
            scope.define_function(
                builder,
                "log1",
                &["uint256 p", "uint256 s", "uint256 t1"],
                None,
            );
            scope.define_function(
                builder,
                "log2",
                &["uint256 p", "uint256 s", "uint256 t1", "uint256 t2"],
                None,
            );
            scope.define_function(
                builder,
                "log3",
                &[
                    "uint256 p",
                    "uint256 s",
                    "uint256 t1",
                    "uint256 t2",
                    "uint256 t3",
                ],
                None,
            );
            scope.define_function(
                builder,
                "log4",
                &[
                    "uint256 p",
                    "uint256 s",
                    "uint256 t1",
                    "uint256 t2",
                    "uint256 t3",
                ],
                None,
            );
            scope.define_function(builder, "lt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "mload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "mod", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "msize", &[], Some("uint256"));
            scope.define_function(builder, "mstore8", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "mstore", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "mul", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "mulmod",
                &["uint256 x", "uint256 y", "uint256 m"],
                Some("uint256"),
            );
            scope.define_function(builder, "not", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "number", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "origin", &[], Some("uint256"));
            scope.define_function(builder, "or", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "pop", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "return", &["uint256 p", "uint256 s"], None);
            scope.define_function(builder, "revert", &["uint256 p", "uint256 s"], None);
            scope.define_function(
                builder,
                "sdiv",
                &["uint256 x", "uint256 y"],
                Some("uint256"),
            );
            scope.define_function(builder, "selfdestruct", &["uint256 a"], None);
            scope.define_function(builder, "sgt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "signextend",
                &["uint256 i", "uint256 x"],
                Some("uint256"),
            );
            scope.define_function(builder, "sload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "slt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "smod",
                &["uint256 x", "uint256 y"],
                Some("uint256"),
            );
            scope.define_function(builder, "sstore", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "stop", &[], None);
            scope.define_function(builder, "sub", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "timestamp", &[], Some("uint256"));
            scope.define_function(builder, "xor", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "keccak256",
                &["uint256 p", "uint256 n"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "returndatacopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "returndatasize", &[], Some("uint256"));
            scope.define_function(
                builder,
                "staticcall",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "create2",
                &["uint256 v", "uint256 p", "uint256 n", "uint256 s"],
                Some("uint256"),
            );
            scope.define_function(builder, "extcodehash", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "sar", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "shl", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "shr", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "chainid", &[], Some("uint256"));
            scope.define_function(builder, "selfbalance", &[], Some("uint256"));
            scope.define_function(builder, "basefee", &[], Some("uint256"));
            scope.define_function(builder, "difficulty", &[], Some("uint256"));
        }
    } else if *version < Version::new(0, 8, 24) {
        // 0.8.18
        // SolidityBuiltIns
        {
            scope.define_function(
                builder,
                "addmod",
                &["uint x", "uint y", "uint k"],
                Some("uint256"),
            );
            scope.define_function(builder, "assert", &["bool condition"], None);
            scope.define_function(builder, "blockhash", &["uint blockNumber"], Some("bytes32"));
            scope.define_function(
                builder,
                "ecrecover",
                &["bytes32 hash", "uint8 v", "bytes32 r", "bytes32 s"],
                Some("address"),
            );
            scope.define_function(builder, "gasleft", &[], Some("uint256"));
            scope.define_function(builder, "keccak256", &["bytes memory"], Some("bytes32"));
            scope.define_function(
                builder,
                "mulmod",
                &["uint x", "uint y", "uint k"],
                Some("uint256"),
            );
            scope.define_function(builder, "require", &["bool condition"], None);
            scope.define_function(
                builder,
                "require",
                &["bool condition", "string memory message"],
                None,
            );
            scope.define_function(builder, "revert", &[], None);
            scope.define_function(builder, "revert", &["string memory reason"], None);
            scope.define_function(builder, "ripemd160", &["bytes memory"], Some("bytes20"));
            scope.define_function(
                builder,
                "selfdestruct",
                &["address payable recipient"],
                None,
            );
            scope.define_function(builder, "sha256", &["bytes memory"], Some("bytes32"));
            let mut type_scope = scope.define_type(builder, "%AbiType");
            type_scope.define_function(
                builder,
                "decode",
                &["bytes memory encodedData", "%Type[] encodedTypesTuple"],
                Some("%Any[]"),
            );
            type_scope.define_function(
                builder,
                "encode",
                &["%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodeCall",
                &[
                    "function() functionPointer",
                    "%Any[] functionArgumentsTuple",
                ],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodePacked",
                &["%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodeWithSelector",
                &["bytes4 selector", "%Any[] functionArgumentsTuple"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodeWithSignature",
                &["string memory signature", "%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "address");
            type_scope.define_field(builder, "balance", "uint256");
            type_scope.define_field(builder, "code", "bytes");
            type_scope.define_field(builder, "codehash", "bytes32");
            type_scope.define_function(
                builder,
                "call",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(
                builder,
                "delegatecall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "send", &["uint256 amount"], Some("bool"));
            type_scope.define_function(
                builder,
                "staticcall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "address payable");
            type_scope.define_field(builder, "balance", "uint256");
            type_scope.define_field(builder, "code", "bytes");
            type_scope.define_field(builder, "codehash", "bytes32");
            type_scope.define_function(
                builder,
                "call",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(
                builder,
                "delegatecall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "send", &["uint256 amount"], Some("bool"));
            type_scope.define_function(
                builder,
                "staticcall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "transfer", &["uint256 amount"], None);
            let mut type_scope = scope.define_type(builder, "%Array");
            type_scope.define_field(builder, "length", "uint256");
            type_scope.define_function(builder, "push", &[], Some("%ValueType"));
            type_scope.define_function(builder, "push", &["%ValueType element"], None);
            type_scope.define_function(builder, "pop", &[], None);
            let mut type_scope = scope.define_type(builder, "%FixedArray");
            type_scope.define_field(builder, "length", "uint256");
            let mut type_scope = scope.define_type(builder, "%BlockType");
            type_scope.define_field(builder, "basefee", "uint256");
            type_scope.define_field(builder, "chainid", "uint256");
            type_scope.define_field(builder, "coinbase", "address payable");
            type_scope.define_field(builder, "difficulty", "uint256");
            type_scope.define_field(builder, "gaslimit", "uint256");
            type_scope.define_field(builder, "number", "uint256");
            type_scope.define_field(builder, "prevrandao", "uint256");
            type_scope.define_field(builder, "timestamp", "uint256");
            let mut type_scope = scope.define_type(builder, "bytes");
            type_scope.define_field(builder, "length", "uint256");
            let mut type_scope = scope.define_type(builder, "%BytesType");
            type_scope.define_function(
                builder,
                "concat",
                &["bytes[] bytesToConcatenate"],
                Some("bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "%CallOptions");
            type_scope.define_field(builder, "gas", "uint256");
            type_scope.define_field(builder, "salt", "uint256");
            type_scope.define_field(builder, "value", "uint256");
            let mut type_scope = scope.define_type(builder, "Error");
            type_scope.define_field(builder, "reason", "string");
            let mut type_scope = scope.define_type(builder, "%ErrorType");
            type_scope.define_field(builder, "selector", "bytes4");
            _ = scope.define_type(builder, "%Function");
            let mut type_scope = scope.define_type(builder, "%ExternalFunction");
            type_scope.define_field(builder, "address", "address");
            type_scope.define_field(builder, "selector", "bytes4");
            let mut type_scope = scope.define_type(builder, "%MessageType");
            type_scope.define_field(builder, "data", "bytes");
            type_scope.define_field(builder, "sender", "address");
            type_scope.define_field(builder, "sig", "bytes4");
            type_scope.define_field(builder, "value", "uint256");
            let mut type_scope = scope.define_type(builder, "Panic");
            type_scope.define_field(builder, "errorCode", "uint256");
            let mut type_scope = scope.define_type(builder, "%StringType");
            type_scope.define_function(
                builder,
                "concat",
                &["string[] stringsToConcatenate"],
                Some("string memory"),
            );
            let mut type_scope = scope.define_type(builder, "%TransactionType");
            type_scope.define_field(builder, "gasprice", "uint256");
            type_scope.define_field(builder, "origin", "address");
            let mut type_scope = scope.define_type(builder, "%ContractTypeType");
            type_scope.define_field(builder, "name", "string");
            type_scope.define_field(builder, "creationCode", "bytes");
            type_scope.define_field(builder, "runtimeCode", "bytes");
            type_scope.define_field(builder, "interfaceId", "bytes4");
            let mut type_scope = scope.define_type(builder, "%InterfaceTypeType");
            type_scope.define_field(builder, "name", "string");
            type_scope.define_field(builder, "interfaceId", "bytes4");
            let mut type_scope = scope.define_type(builder, "%IntTypeType");
            type_scope.define_field(builder, "min", "int256");
            type_scope.define_field(builder, "max", "int256");
            let mut type_scope = scope.define_type(builder, "%UserDefinedValueType");
            type_scope.define_function(
                builder,
                "wrap",
                &["%WrappedType elementaryType"],
                Some("%UserType"),
            );
            type_scope.define_function(
                builder,
                "unwrap",
                &["%UserType userType"],
                Some("%WrappedType"),
            );
            scope.define_field(builder, "%_", "%Function");
            scope.define_field(builder, "abi", "%AbiType");
            scope.define_field(builder, "block", "%BlockType");
            scope.define_field(builder, "bytes", "%BytesType");
            scope.define_field(builder, "msg", "%MessageType");
            scope.define_field(builder, "string", "%StringType");
            scope.define_field(builder, "tx", "%TransactionType");
        }
        // YulBuiltIns
        {
            let mut scope = scope.new_context(builder, "@yul");
            let mut type_scope = scope.define_type(builder, "%YulExternal");
            type_scope.define_field(builder, "slot", "uint256");
            type_scope.define_field(builder, "offset", "uint256");
            type_scope.define_field(builder, "length", "uint256");
            scope.define_function(builder, "add", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "addmod",
                &["uint256 x", "uint256 y", "uint256 m"],
                Some("uint256"),
            );
            scope.define_function(builder, "address", &[], Some("uint256"));
            scope.define_function(builder, "and", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "balance", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "blockhash", &["uint256 b"], Some("uint256"));
            scope.define_function(
                builder,
                "byte",
                &["uint256 n", "uint256 x"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "callcode",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 v",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "calldatacopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "calldataload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "calldatasize", &[], Some("uint256"));
            scope.define_function(builder, "caller", &[], Some("uint256"));
            scope.define_function(
                builder,
                "call",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 v",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(builder, "callvalue", &[], Some("uint256"));
            scope.define_function(
                builder,
                "codecopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "codesize", &[], Some("uint256"));
            scope.define_function(builder, "coinbase", &[], Some("uint256"));
            scope.define_function(
                builder,
                "create",
                &["uint256 v", "uint256 p", "uint256 n"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "delegatecall",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(builder, "div", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "eq", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "exp", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "extcodecopy",
                &["uint256 a", "uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "extcodesize", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "gas", &[], Some("uint256"));
            scope.define_function(builder, "gaslimit", &[], Some("uint256"));
            scope.define_function(builder, "gasprice", &[], Some("uint256"));
            scope.define_function(builder, "gt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "invalid", &[], None);
            scope.define_function(builder, "iszero", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "log0", &["uint256 p", "uint256 s"], None);
            scope.define_function(
                builder,
                "log1",
                &["uint256 p", "uint256 s", "uint256 t1"],
                None,
            );
            scope.define_function(
                builder,
                "log2",
                &["uint256 p", "uint256 s", "uint256 t1", "uint256 t2"],
                None,
            );
            scope.define_function(
                builder,
                "log3",
                &[
                    "uint256 p",
                    "uint256 s",
                    "uint256 t1",
                    "uint256 t2",
                    "uint256 t3",
                ],
                None,
            );
            scope.define_function(
                builder,
                "log4",
                &[
                    "uint256 p",
                    "uint256 s",
                    "uint256 t1",
                    "uint256 t2",
                    "uint256 t3",
                ],
                None,
            );
            scope.define_function(builder, "lt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "mload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "mod", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "msize", &[], Some("uint256"));
            scope.define_function(builder, "mstore8", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "mstore", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "mul", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "mulmod",
                &["uint256 x", "uint256 y", "uint256 m"],
                Some("uint256"),
            );
            scope.define_function(builder, "not", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "number", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "origin", &[], Some("uint256"));
            scope.define_function(builder, "or", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "pop", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "return", &["uint256 p", "uint256 s"], None);
            scope.define_function(builder, "revert", &["uint256 p", "uint256 s"], None);
            scope.define_function(
                builder,
                "sdiv",
                &["uint256 x", "uint256 y"],
                Some("uint256"),
            );
            scope.define_function(builder, "selfdestruct", &["uint256 a"], None);
            scope.define_function(builder, "sgt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "signextend",
                &["uint256 i", "uint256 x"],
                Some("uint256"),
            );
            scope.define_function(builder, "sload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "slt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "smod",
                &["uint256 x", "uint256 y"],
                Some("uint256"),
            );
            scope.define_function(builder, "sstore", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "stop", &[], None);
            scope.define_function(builder, "sub", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "timestamp", &[], Some("uint256"));
            scope.define_function(builder, "xor", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "keccak256",
                &["uint256 p", "uint256 n"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "returndatacopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "returndatasize", &[], Some("uint256"));
            scope.define_function(
                builder,
                "staticcall",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "create2",
                &["uint256 v", "uint256 p", "uint256 n", "uint256 s"],
                Some("uint256"),
            );
            scope.define_function(builder, "extcodehash", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "sar", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "shl", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "shr", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "chainid", &[], Some("uint256"));
            scope.define_function(builder, "selfbalance", &[], Some("uint256"));
            scope.define_function(builder, "basefee", &[], Some("uint256"));
            scope.define_function(builder, "prevrandao", &[], Some("uint256"));
        }
    } else if *version < Version::new(0, 8, 26) {
        // 0.8.24
        // SolidityBuiltIns
        {
            scope.define_function(
                builder,
                "addmod",
                &["uint x", "uint y", "uint k"],
                Some("uint256"),
            );
            scope.define_function(builder, "assert", &["bool condition"], None);
            scope.define_function(builder, "blockhash", &["uint blockNumber"], Some("bytes32"));
            scope.define_function(builder, "blobhash", &["uint index"], Some("bytes32"));
            scope.define_function(
                builder,
                "ecrecover",
                &["bytes32 hash", "uint8 v", "bytes32 r", "bytes32 s"],
                Some("address"),
            );
            scope.define_function(builder, "gasleft", &[], Some("uint256"));
            scope.define_function(builder, "keccak256", &["bytes memory"], Some("bytes32"));
            scope.define_function(
                builder,
                "mulmod",
                &["uint x", "uint y", "uint k"],
                Some("uint256"),
            );
            scope.define_function(builder, "require", &["bool condition"], None);
            scope.define_function(
                builder,
                "require",
                &["bool condition", "string memory message"],
                None,
            );
            scope.define_function(builder, "revert", &[], None);
            scope.define_function(builder, "revert", &["string memory reason"], None);
            scope.define_function(builder, "ripemd160", &["bytes memory"], Some("bytes20"));
            scope.define_function(
                builder,
                "selfdestruct",
                &["address payable recipient"],
                None,
            );
            scope.define_function(builder, "sha256", &["bytes memory"], Some("bytes32"));
            let mut type_scope = scope.define_type(builder, "%AbiType");
            type_scope.define_function(
                builder,
                "decode",
                &["bytes memory encodedData", "%Type[] encodedTypesTuple"],
                Some("%Any[]"),
            );
            type_scope.define_function(
                builder,
                "encode",
                &["%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodeCall",
                &[
                    "function() functionPointer",
                    "%Any[] functionArgumentsTuple",
                ],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodePacked",
                &["%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodeWithSelector",
                &["bytes4 selector", "%Any[] functionArgumentsTuple"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodeWithSignature",
                &["string memory signature", "%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "address");
            type_scope.define_field(builder, "balance", "uint256");
            type_scope.define_field(builder, "code", "bytes");
            type_scope.define_field(builder, "codehash", "bytes32");
            type_scope.define_function(
                builder,
                "call",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(
                builder,
                "delegatecall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "send", &["uint256 amount"], Some("bool"));
            type_scope.define_function(
                builder,
                "staticcall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "address payable");
            type_scope.define_field(builder, "balance", "uint256");
            type_scope.define_field(builder, "code", "bytes");
            type_scope.define_field(builder, "codehash", "bytes32");
            type_scope.define_function(
                builder,
                "call",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(
                builder,
                "delegatecall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "send", &["uint256 amount"], Some("bool"));
            type_scope.define_function(
                builder,
                "staticcall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "transfer", &["uint256 amount"], None);
            let mut type_scope = scope.define_type(builder, "%Array");
            type_scope.define_field(builder, "length", "uint256");
            type_scope.define_function(builder, "push", &[], Some("%ValueType"));
            type_scope.define_function(builder, "push", &["%ValueType element"], None);
            type_scope.define_function(builder, "pop", &[], None);
            let mut type_scope = scope.define_type(builder, "%FixedArray");
            type_scope.define_field(builder, "length", "uint256");
            let mut type_scope = scope.define_type(builder, "%BlockType");
            type_scope.define_field(builder, "basefee", "uint256");
            type_scope.define_field(builder, "blobbasefee", "uint256");
            type_scope.define_field(builder, "chainid", "uint256");
            type_scope.define_field(builder, "coinbase", "address payable");
            type_scope.define_field(builder, "difficulty", "uint256");
            type_scope.define_field(builder, "gaslimit", "uint256");
            type_scope.define_field(builder, "number", "uint256");
            type_scope.define_field(builder, "prevrandao", "uint256");
            type_scope.define_field(builder, "timestamp", "uint256");
            let mut type_scope = scope.define_type(builder, "bytes");
            type_scope.define_field(builder, "length", "uint256");
            let mut type_scope = scope.define_type(builder, "%BytesType");
            type_scope.define_function(
                builder,
                "concat",
                &["bytes[] bytesToConcatenate"],
                Some("bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "%CallOptions");
            type_scope.define_field(builder, "gas", "uint256");
            type_scope.define_field(builder, "salt", "uint256");
            type_scope.define_field(builder, "value", "uint256");
            let mut type_scope = scope.define_type(builder, "Error");
            type_scope.define_field(builder, "reason", "string");
            let mut type_scope = scope.define_type(builder, "%ErrorType");
            type_scope.define_field(builder, "selector", "bytes4");
            _ = scope.define_type(builder, "%Function");
            let mut type_scope = scope.define_type(builder, "%ExternalFunction");
            type_scope.define_field(builder, "address", "address");
            type_scope.define_field(builder, "selector", "bytes4");
            let mut type_scope = scope.define_type(builder, "%MessageType");
            type_scope.define_field(builder, "data", "bytes");
            type_scope.define_field(builder, "sender", "address");
            type_scope.define_field(builder, "sig", "bytes4");
            type_scope.define_field(builder, "value", "uint256");
            let mut type_scope = scope.define_type(builder, "Panic");
            type_scope.define_field(builder, "errorCode", "uint256");
            let mut type_scope = scope.define_type(builder, "%StringType");
            type_scope.define_function(
                builder,
                "concat",
                &["string[] stringsToConcatenate"],
                Some("string memory"),
            );
            let mut type_scope = scope.define_type(builder, "%TransactionType");
            type_scope.define_field(builder, "gasprice", "uint256");
            type_scope.define_field(builder, "origin", "address");
            let mut type_scope = scope.define_type(builder, "%ContractTypeType");
            type_scope.define_field(builder, "name", "string");
            type_scope.define_field(builder, "creationCode", "bytes");
            type_scope.define_field(builder, "runtimeCode", "bytes");
            type_scope.define_field(builder, "interfaceId", "bytes4");
            let mut type_scope = scope.define_type(builder, "%InterfaceTypeType");
            type_scope.define_field(builder, "name", "string");
            type_scope.define_field(builder, "interfaceId", "bytes4");
            let mut type_scope = scope.define_type(builder, "%IntTypeType");
            type_scope.define_field(builder, "min", "int256");
            type_scope.define_field(builder, "max", "int256");
            let mut type_scope = scope.define_type(builder, "%UserDefinedValueType");
            type_scope.define_function(
                builder,
                "wrap",
                &["%WrappedType elementaryType"],
                Some("%UserType"),
            );
            type_scope.define_function(
                builder,
                "unwrap",
                &["%UserType userType"],
                Some("%WrappedType"),
            );
            scope.define_field(builder, "%_", "%Function");
            scope.define_field(builder, "abi", "%AbiType");
            scope.define_field(builder, "block", "%BlockType");
            scope.define_field(builder, "bytes", "%BytesType");
            scope.define_field(builder, "msg", "%MessageType");
            scope.define_field(builder, "string", "%StringType");
            scope.define_field(builder, "tx", "%TransactionType");
        }
        // YulBuiltIns
        {
            let mut scope = scope.new_context(builder, "@yul");
            let mut type_scope = scope.define_type(builder, "%YulExternal");
            type_scope.define_field(builder, "slot", "uint256");
            type_scope.define_field(builder, "offset", "uint256");
            type_scope.define_field(builder, "length", "uint256");
            scope.define_function(builder, "add", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "addmod",
                &["uint256 x", "uint256 y", "uint256 m"],
                Some("uint256"),
            );
            scope.define_function(builder, "address", &[], Some("uint256"));
            scope.define_function(builder, "and", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "balance", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "blockhash", &["uint256 b"], Some("uint256"));
            scope.define_function(
                builder,
                "byte",
                &["uint256 n", "uint256 x"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "callcode",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 v",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "calldatacopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "calldataload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "calldatasize", &[], Some("uint256"));
            scope.define_function(builder, "caller", &[], Some("uint256"));
            scope.define_function(
                builder,
                "call",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 v",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(builder, "callvalue", &[], Some("uint256"));
            scope.define_function(
                builder,
                "codecopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "codesize", &[], Some("uint256"));
            scope.define_function(builder, "coinbase", &[], Some("uint256"));
            scope.define_function(
                builder,
                "create",
                &["uint256 v", "uint256 p", "uint256 n"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "delegatecall",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(builder, "div", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "eq", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "exp", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "extcodecopy",
                &["uint256 a", "uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "extcodesize", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "gas", &[], Some("uint256"));
            scope.define_function(builder, "gaslimit", &[], Some("uint256"));
            scope.define_function(builder, "gasprice", &[], Some("uint256"));
            scope.define_function(builder, "gt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "invalid", &[], None);
            scope.define_function(builder, "iszero", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "log0", &["uint256 p", "uint256 s"], None);
            scope.define_function(
                builder,
                "log1",
                &["uint256 p", "uint256 s", "uint256 t1"],
                None,
            );
            scope.define_function(
                builder,
                "log2",
                &["uint256 p", "uint256 s", "uint256 t1", "uint256 t2"],
                None,
            );
            scope.define_function(
                builder,
                "log3",
                &[
                    "uint256 p",
                    "uint256 s",
                    "uint256 t1",
                    "uint256 t2",
                    "uint256 t3",
                ],
                None,
            );
            scope.define_function(
                builder,
                "log4",
                &[
                    "uint256 p",
                    "uint256 s",
                    "uint256 t1",
                    "uint256 t2",
                    "uint256 t3",
                ],
                None,
            );
            scope.define_function(builder, "lt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "mload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "mod", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "msize", &[], Some("uint256"));
            scope.define_function(builder, "mstore8", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "mstore", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "mul", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "mulmod",
                &["uint256 x", "uint256 y", "uint256 m"],
                Some("uint256"),
            );
            scope.define_function(builder, "not", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "number", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "origin", &[], Some("uint256"));
            scope.define_function(builder, "or", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "pop", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "return", &["uint256 p", "uint256 s"], None);
            scope.define_function(builder, "revert", &["uint256 p", "uint256 s"], None);
            scope.define_function(
                builder,
                "sdiv",
                &["uint256 x", "uint256 y"],
                Some("uint256"),
            );
            scope.define_function(builder, "selfdestruct", &["uint256 a"], None);
            scope.define_function(builder, "sgt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "signextend",
                &["uint256 i", "uint256 x"],
                Some("uint256"),
            );
            scope.define_function(builder, "sload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "slt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "smod",
                &["uint256 x", "uint256 y"],
                Some("uint256"),
            );
            scope.define_function(builder, "sstore", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "stop", &[], None);
            scope.define_function(builder, "sub", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "timestamp", &[], Some("uint256"));
            scope.define_function(builder, "xor", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "keccak256",
                &["uint256 p", "uint256 n"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "returndatacopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "returndatasize", &[], Some("uint256"));
            scope.define_function(
                builder,
                "staticcall",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "create2",
                &["uint256 v", "uint256 p", "uint256 n", "uint256 s"],
                Some("uint256"),
            );
            scope.define_function(builder, "extcodehash", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "sar", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "shl", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "shr", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "chainid", &[], Some("uint256"));
            scope.define_function(builder, "selfbalance", &[], Some("uint256"));
            scope.define_function(builder, "basefee", &[], Some("uint256"));
            scope.define_function(builder, "prevrandao", &[], Some("uint256"));
            scope.define_function(builder, "blobbasefee", &[], Some("uint256"));
            scope.define_function(builder, "blobhash", &["uint256 i"], Some("uint256"));
            scope.define_function(builder, "tload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "tstore", &["uint256 p", "uint256 v"], None);
            scope.define_function(
                builder,
                "mcopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
        }
    } else {
        // 0.8.26
        // SolidityBuiltIns
        {
            scope.define_function(
                builder,
                "addmod",
                &["uint x", "uint y", "uint k"],
                Some("uint256"),
            );
            scope.define_function(builder, "assert", &["bool condition"], None);
            scope.define_function(builder, "blockhash", &["uint blockNumber"], Some("bytes32"));
            scope.define_function(builder, "blobhash", &["uint index"], Some("bytes32"));
            scope.define_function(
                builder,
                "ecrecover",
                &["bytes32 hash", "uint8 v", "bytes32 r", "bytes32 s"],
                Some("address"),
            );
            scope.define_function(builder, "gasleft", &[], Some("uint256"));
            scope.define_function(builder, "keccak256", &["bytes memory"], Some("bytes32"));
            scope.define_function(
                builder,
                "mulmod",
                &["uint x", "uint y", "uint k"],
                Some("uint256"),
            );
            scope.define_function(builder, "require", &["bool condition"], None);
            scope.define_function(
                builder,
                "require",
                &["bool condition", "string memory message"],
                None,
            );
            scope.define_function(builder, "require", &["bool condition", "Error error"], None);
            scope.define_function(builder, "revert", &[], None);
            scope.define_function(builder, "revert", &["string memory reason"], None);
            scope.define_function(builder, "ripemd160", &["bytes memory"], Some("bytes20"));
            scope.define_function(
                builder,
                "selfdestruct",
                &["address payable recipient"],
                None,
            );
            scope.define_function(builder, "sha256", &["bytes memory"], Some("bytes32"));
            let mut type_scope = scope.define_type(builder, "%AbiType");
            type_scope.define_function(
                builder,
                "decode",
                &["bytes memory encodedData", "%Type[] encodedTypesTuple"],
                Some("%Any[]"),
            );
            type_scope.define_function(
                builder,
                "encode",
                &["%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodeCall",
                &[
                    "function() functionPointer",
                    "%Any[] functionArgumentsTuple",
                ],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodePacked",
                &["%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodeWithSelector",
                &["bytes4 selector", "%Any[] functionArgumentsTuple"],
                Some("bytes memory"),
            );
            type_scope.define_function(
                builder,
                "encodeWithSignature",
                &["string memory signature", "%Any[] valuesToEncode"],
                Some("bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "address");
            type_scope.define_field(builder, "balance", "uint256");
            type_scope.define_field(builder, "code", "bytes");
            type_scope.define_field(builder, "codehash", "bytes32");
            type_scope.define_function(
                builder,
                "call",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(
                builder,
                "delegatecall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "send", &["uint256 amount"], Some("bool"));
            type_scope.define_function(
                builder,
                "staticcall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "address payable");
            type_scope.define_field(builder, "balance", "uint256");
            type_scope.define_field(builder, "code", "bytes");
            type_scope.define_field(builder, "codehash", "bytes32");
            type_scope.define_function(
                builder,
                "call",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(
                builder,
                "delegatecall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "send", &["uint256 amount"], Some("bool"));
            type_scope.define_function(
                builder,
                "staticcall",
                &["bytes memory"],
                Some("bool, bytes memory"),
            );
            type_scope.define_function(builder, "transfer", &["uint256 amount"], None);
            let mut type_scope = scope.define_type(builder, "%Array");
            type_scope.define_field(builder, "length", "uint256");
            type_scope.define_function(builder, "push", &[], Some("%ValueType"));
            type_scope.define_function(builder, "push", &["%ValueType element"], None);
            type_scope.define_function(builder, "pop", &[], None);
            let mut type_scope = scope.define_type(builder, "%FixedArray");
            type_scope.define_field(builder, "length", "uint256");
            let mut type_scope = scope.define_type(builder, "%BlockType");
            type_scope.define_field(builder, "basefee", "uint256");
            type_scope.define_field(builder, "blobbasefee", "uint256");
            type_scope.define_field(builder, "chainid", "uint256");
            type_scope.define_field(builder, "coinbase", "address payable");
            type_scope.define_field(builder, "difficulty", "uint256");
            type_scope.define_field(builder, "gaslimit", "uint256");
            type_scope.define_field(builder, "number", "uint256");
            type_scope.define_field(builder, "prevrandao", "uint256");
            type_scope.define_field(builder, "timestamp", "uint256");
            let mut type_scope = scope.define_type(builder, "bytes");
            type_scope.define_field(builder, "length", "uint256");
            let mut type_scope = scope.define_type(builder, "%BytesType");
            type_scope.define_function(
                builder,
                "concat",
                &["bytes[] bytesToConcatenate"],
                Some("bytes memory"),
            );
            let mut type_scope = scope.define_type(builder, "%CallOptions");
            type_scope.define_field(builder, "gas", "uint256");
            type_scope.define_field(builder, "salt", "uint256");
            type_scope.define_field(builder, "value", "uint256");
            let mut type_scope = scope.define_type(builder, "Error");
            type_scope.define_field(builder, "reason", "string");
            let mut type_scope = scope.define_type(builder, "%ErrorType");
            type_scope.define_field(builder, "selector", "bytes4");
            _ = scope.define_type(builder, "%Function");
            let mut type_scope = scope.define_type(builder, "%ExternalFunction");
            type_scope.define_field(builder, "address", "address");
            type_scope.define_field(builder, "selector", "bytes4");
            let mut type_scope = scope.define_type(builder, "%MessageType");
            type_scope.define_field(builder, "data", "bytes");
            type_scope.define_field(builder, "sender", "address");
            type_scope.define_field(builder, "sig", "bytes4");
            type_scope.define_field(builder, "value", "uint256");
            let mut type_scope = scope.define_type(builder, "Panic");
            type_scope.define_field(builder, "errorCode", "uint256");
            let mut type_scope = scope.define_type(builder, "%StringType");
            type_scope.define_function(
                builder,
                "concat",
                &["string[] stringsToConcatenate"],
                Some("string memory"),
            );
            let mut type_scope = scope.define_type(builder, "%TransactionType");
            type_scope.define_field(builder, "gasprice", "uint256");
            type_scope.define_field(builder, "origin", "address");
            let mut type_scope = scope.define_type(builder, "%ContractTypeType");
            type_scope.define_field(builder, "name", "string");
            type_scope.define_field(builder, "creationCode", "bytes");
            type_scope.define_field(builder, "runtimeCode", "bytes");
            type_scope.define_field(builder, "interfaceId", "bytes4");
            let mut type_scope = scope.define_type(builder, "%InterfaceTypeType");
            type_scope.define_field(builder, "name", "string");
            type_scope.define_field(builder, "interfaceId", "bytes4");
            let mut type_scope = scope.define_type(builder, "%IntTypeType");
            type_scope.define_field(builder, "min", "int256");
            type_scope.define_field(builder, "max", "int256");
            let mut type_scope = scope.define_type(builder, "%UserDefinedValueType");
            type_scope.define_function(
                builder,
                "wrap",
                &["%WrappedType elementaryType"],
                Some("%UserType"),
            );
            type_scope.define_function(
                builder,
                "unwrap",
                &["%UserType userType"],
                Some("%WrappedType"),
            );
            scope.define_field(builder, "%_", "%Function");
            scope.define_field(builder, "abi", "%AbiType");
            scope.define_field(builder, "block", "%BlockType");
            scope.define_field(builder, "bytes", "%BytesType");
            scope.define_field(builder, "msg", "%MessageType");
            scope.define_field(builder, "string", "%StringType");
            scope.define_field(builder, "tx", "%TransactionType");
        }
        // YulBuiltIns
        {
            let mut scope = scope.new_context(builder, "@yul");
            let mut type_scope = scope.define_type(builder, "%YulExternal");
            type_scope.define_field(builder, "slot", "uint256");
            type_scope.define_field(builder, "offset", "uint256");
            type_scope.define_field(builder, "length", "uint256");
            scope.define_function(builder, "add", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "addmod",
                &["uint256 x", "uint256 y", "uint256 m"],
                Some("uint256"),
            );
            scope.define_function(builder, "address", &[], Some("uint256"));
            scope.define_function(builder, "and", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "balance", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "blockhash", &["uint256 b"], Some("uint256"));
            scope.define_function(
                builder,
                "byte",
                &["uint256 n", "uint256 x"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "callcode",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 v",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "calldatacopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "calldataload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "calldatasize", &[], Some("uint256"));
            scope.define_function(builder, "caller", &[], Some("uint256"));
            scope.define_function(
                builder,
                "call",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 v",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(builder, "callvalue", &[], Some("uint256"));
            scope.define_function(
                builder,
                "codecopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "codesize", &[], Some("uint256"));
            scope.define_function(builder, "coinbase", &[], Some("uint256"));
            scope.define_function(
                builder,
                "create",
                &["uint256 v", "uint256 p", "uint256 n"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "delegatecall",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(builder, "div", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "eq", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "exp", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "extcodecopy",
                &["uint256 a", "uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "extcodesize", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "gas", &[], Some("uint256"));
            scope.define_function(builder, "gaslimit", &[], Some("uint256"));
            scope.define_function(builder, "gasprice", &[], Some("uint256"));
            scope.define_function(builder, "gt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "invalid", &[], None);
            scope.define_function(builder, "iszero", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "log0", &["uint256 p", "uint256 s"], None);
            scope.define_function(
                builder,
                "log1",
                &["uint256 p", "uint256 s", "uint256 t1"],
                None,
            );
            scope.define_function(
                builder,
                "log2",
                &["uint256 p", "uint256 s", "uint256 t1", "uint256 t2"],
                None,
            );
            scope.define_function(
                builder,
                "log3",
                &[
                    "uint256 p",
                    "uint256 s",
                    "uint256 t1",
                    "uint256 t2",
                    "uint256 t3",
                ],
                None,
            );
            scope.define_function(
                builder,
                "log4",
                &[
                    "uint256 p",
                    "uint256 s",
                    "uint256 t1",
                    "uint256 t2",
                    "uint256 t3",
                ],
                None,
            );
            scope.define_function(builder, "lt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "mload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "mod", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "msize", &[], Some("uint256"));
            scope.define_function(builder, "mstore8", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "mstore", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "mul", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "mulmod",
                &["uint256 x", "uint256 y", "uint256 m"],
                Some("uint256"),
            );
            scope.define_function(builder, "not", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "number", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "origin", &[], Some("uint256"));
            scope.define_function(builder, "or", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "pop", &["uint256 x"], Some("uint256"));
            scope.define_function(builder, "return", &["uint256 p", "uint256 s"], None);
            scope.define_function(builder, "revert", &["uint256 p", "uint256 s"], None);
            scope.define_function(
                builder,
                "sdiv",
                &["uint256 x", "uint256 y"],
                Some("uint256"),
            );
            scope.define_function(builder, "selfdestruct", &["uint256 a"], None);
            scope.define_function(builder, "sgt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "signextend",
                &["uint256 i", "uint256 x"],
                Some("uint256"),
            );
            scope.define_function(builder, "sload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "slt", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "smod",
                &["uint256 x", "uint256 y"],
                Some("uint256"),
            );
            scope.define_function(builder, "sstore", &["uint256 p", "uint256 v"], None);
            scope.define_function(builder, "stop", &[], None);
            scope.define_function(builder, "sub", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "timestamp", &[], Some("uint256"));
            scope.define_function(builder, "xor", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(
                builder,
                "keccak256",
                &["uint256 p", "uint256 n"],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "returndatacopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
            scope.define_function(builder, "returndatasize", &[], Some("uint256"));
            scope.define_function(
                builder,
                "staticcall",
                &[
                    "uint256 g",
                    "uint256 a",
                    "uint256 in_",
                    "uint256 insize",
                    "uint256 out",
                    "uint256 outsize",
                ],
                Some("uint256"),
            );
            scope.define_function(
                builder,
                "create2",
                &["uint256 v", "uint256 p", "uint256 n", "uint256 s"],
                Some("uint256"),
            );
            scope.define_function(builder, "extcodehash", &["uint256 a"], Some("uint256"));
            scope.define_function(builder, "sar", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "shl", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "shr", &["uint256 x", "uint256 y"], Some("uint256"));
            scope.define_function(builder, "chainid", &[], Some("uint256"));
            scope.define_function(builder, "selfbalance", &[], Some("uint256"));
            scope.define_function(builder, "basefee", &[], Some("uint256"));
            scope.define_function(builder, "prevrandao", &[], Some("uint256"));
            scope.define_function(builder, "blobbasefee", &[], Some("uint256"));
            scope.define_function(builder, "blobhash", &["uint256 i"], Some("uint256"));
            scope.define_function(builder, "tload", &["uint256 p"], Some("uint256"));
            scope.define_function(builder, "tstore", &["uint256 p", "uint256 v"], None);
            scope.define_function(
                builder,
                "mcopy",
                &["uint256 t", "uint256 f", "uint256 s"],
                None,
            );
        }
    }
}
