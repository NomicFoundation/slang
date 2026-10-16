// https://github.com/argotorg/solidity/blob/v0.8.0/test/libsolidity/syntaxTests/metaTypes/noArgForType.sol

contract Test {
    function creation() public pure returns (bytes memory) {
        type();
    }
}
