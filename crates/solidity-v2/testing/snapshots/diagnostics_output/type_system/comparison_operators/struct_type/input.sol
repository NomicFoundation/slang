// https://github.com/argotorg/solidity/blob/8471cf2ff005320b69535ee923e75edb569927d4/test/libsolidity/syntaxTests/nameAndTypeResolution/203_struct_reference_compare_operators.sol#L10

contract test {
  struct s {uint a;}
  s x;
  s y;
  fallback() external {
    x == y;
  }
}
