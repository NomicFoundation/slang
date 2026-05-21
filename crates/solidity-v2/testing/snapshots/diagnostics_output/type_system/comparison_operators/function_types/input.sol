
contract C {
    function inequality() public returns (bool ret) {
        this.f > this.f;
        f < f;
    }
    
    function external_test_function() external {}
    function internal_test_function() internal {}

    function comparison_operator_between_internal_and_external_function_pointers() external returns (bool) {
        function () external external_function_pointer_local = this.external_test_function;
        function () internal internal_function_pointer_local = internal_test_function;

        internal_function_pointer_local != external_function_pointer_local;
        internal_test_function != this.external_test_function;
    }
    
    function external_test_function1(uint num) external {}
    function external_test_function2(bool val) external {}

    function comparison_operator_between_external_function_pointers_with_different_arguments() external returns (bool) {
        function () external external_function_pointer_local1 = this.external_test_function1;
        function () external external_function_pointer_local2 = this.external_test_function2;

        external_function_pointer_local2 != external_function_pointer_local1;
        this.external_test_function2 != this.external_test_function1;
    }


}
