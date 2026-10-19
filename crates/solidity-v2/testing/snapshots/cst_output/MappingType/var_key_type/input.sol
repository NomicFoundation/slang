// https://github.com/argotorg/solidity/blob/v0.8.0/test/libsolidity/syntaxTests/parsing/variable_definition_in_mapping.sol

contract test {
    function fun() {
        mapping(var=>bytes32) d;
    }
}
