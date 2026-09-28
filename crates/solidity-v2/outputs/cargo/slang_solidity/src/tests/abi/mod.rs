mod eip712;
mod entries;
mod getters;
mod interface_abi;
mod interface_id;
mod internal_signature;
mod internal_type;
mod library;
mod selectors;
mod storage_layout;
mod type_conversion;

use super::fixtures;
use crate::define_fixture;

define_fixture!(
    AbiWithTuples,
    file: "main.sol", r#"
pragma solidity *;
contract Test {
    struct S { uint a; uint[] b; T[] c; }
    struct T { uint x; uint y; }

    function f(S memory, T memory, uint x) public pure {}
    function g() public pure returns (S memory s, T memory t, uint) {}

    T public t;  // getter returns (uint x, uint y)
    function t_components() public view returns (uint, uint) {
        return (t.x, t.y);
    }
    function t_struct() public view returns (T memory) {
        return t;
    }
}
"#,
);

define_fixture!(
    FullAbi,
    file: "main.sol", r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8;

interface ITest {
    error SomethingWrong(string);
    event BaseEvent(uint a, string m) anonymous;
}

contract Base {
    uint[] public xs;
    bool a;
}

contract Test is ITest, Base {
    bytes32 public b;
    constructor() { b = hex"12345678901234567890123456789012"; }
    event Event(uint indexed a, bytes32 b);
    error InsufficientBalance(uint256 available, uint256 required);
    function foo(uint a) public { emit Event(a, b); }
    receive() external payable { }
    fallback() external { }
}
"#,
);

define_fixture!(
    LibraryAbi,
    file: "main.sol", r#"
pragma solidity *;
library L {
    struct S { mapping(uint => uint) m; }
    type U is uint64;

    function f(S storage s, uint x) external returns (uint) {}
    function g(uint[] storage xs) public {}
    function h(uint x) external pure returns (uint) {}
    function i(uint x) internal pure returns (uint) {}
    function j(U u) external pure {}
}
"#,
);

define_fixture!(
    LibraryUdvtAbi,
    file: "main.sol", r#"
pragma solidity *;
library L {
    type U is uint64;

    function k(U[] memory xs) external pure {}
    function q(U[2] memory xs) external pure {}
    function m(mapping(uint256 => U) storage x) external {}
}
"#,
);

define_fixture!(
    LibraryMembers,
    file: "main.sol", r#"
pragma solidity ^0.8.0;
library LJ {
    uint256 public constant X = 1;
    error E();
    event Ev();
    function view_fn(uint256 x) external view returns (uint256) {}
    function mut_fn(uint256 x) external returns (uint256) {}
}
"#,
);

define_fixture!(
    AbstractContract,
    file: "main.sol", r#"
pragma solidity ^0.8.0;
abstract contract AB {
    constructor(uint256 seed) {}
    function extra() external virtual;
}
"#,
);

define_fixture!(
    LibraryStorageSignature,
    file: "main.sol", r#"
pragma solidity ^0.8.0;
library L {
    struct S { uint256 x; }
    function slot(uint256 k) external view returns (S storage s) { assembly { s.slot := k } }
    function peek(S storage s) external view returns (uint256) { return s.x; }
    function peekArr(uint256[] storage xs) public view returns (uint256) { return xs.length; }
    function value(uint256 k) external view returns (uint256) { return k; }
}
"#,
);

define_fixture!(
    NamedMappings,
    file: "main.sol", r#"
pragma solidity ^0.8.18;
contract Named {
    struct S { uint256 x; uint256[] ys; }
    mapping(address funder => uint256 amountFunded) public addressToAmountFunded;
    mapping(address owner => mapping(uint256 id => bool ok)) public nested;
    mapping(uint256 key => S) public structs;
    mapping(uint256 key => S[] items) public structArrays;
    mapping(uint256 key => uint256[] values) public arrays;
    mapping(uint256 key => uint256[][] values) public arrays2;
    mapping(uint256 key => mapping(uint256 => uint256[] inner)) public throughArray;
    mapping(uint256 key => bytes blob) public bytesValue;
    mapping(uint256 key => uint256[] values)[] public outerArray;
    mapping(uint256 => uint256 value) public unnamedKey;
    uint256[] public plain;
}
"#,
);

define_fixture!(
    FunctionsNamedReceiveAndFallback,
    file: "main.sol", r#"
pragma solidity ^0.8.0;
contract C {
    receive() external payable {}
    fallback() external {}
    function receive(uint256 x) external pure returns (uint256) { return x; }
    function fallback() external payable {}
}
"#,
);

define_fixture!(
    InterfaceHierarchy,
    file: "main.sol", r#"
pragma solidity ^0.8.0;
interface IBase {
    error Denied(address who);
    event Moved(uint256 indexed amount);
    function balance() external view returns (uint256);
    function move(uint256 amount) external;
}
interface IDerived is IBase {
    event Paused();
    function move(uint256 amount) external override;
    function pause() external;
    receive() external payable;
    fallback() external;
}
"#,
);
