// https://github.com/argotorg/solidity/blob/8471cf2ff005320b69535ee923e75edb569927d4/test/libsolidity/syntaxTests/nameAndTypeResolution/202_bytes_reference_compare_operators.sol#L3

contract test { bytes a; bytes b; fallback() external { a == b; } }
