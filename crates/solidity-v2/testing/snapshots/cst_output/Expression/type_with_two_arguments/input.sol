// https://github.com/argotorg/solidity/blob/v0.8.0/test/libsolidity/syntaxTests/metaTypes/tooManyArgsForType.sol
// Uses type names instead of `type(1, 2)`, so that parsing fails on the second argument.

contract Test {
    function creation() public pure returns (bytes memory) {
        type(uint, uint);
    }
}
