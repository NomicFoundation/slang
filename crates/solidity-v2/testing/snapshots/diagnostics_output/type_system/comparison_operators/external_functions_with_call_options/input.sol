// https://github.com/argotorg/solidity/blob/8471cf2ff005320b69535ee923e75edb569927d4/test/libsolidity/syntaxTests/functionTypes/comparison_operator_for_external_functions_with_call_options.sol#L11

// TODO: This test should fail
contract C {
    function external_test_function() payable external {}
    function comparison_operator_for_external_function_with_extra_slots() external returns (bool) {
        return (
            (this.external_test_function{value: 4} == this.external_test_function) &&
            (this.external_test_function{value: 4} == this.external_test_function{value: 4})
        );
    }
}
