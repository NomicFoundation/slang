contract Test {
    function memory_internal(bytes memory) internal {}
    function memory_external(bytes memory) external {}
    function memory_private(bytes memory) private {}
    function memory_public(bytes memory) public {}

    function calldata_internal(bytes calldata) internal {}
    function calldata_external(bytes calldata) external {}
    function calldata_private(bytes calldata) private {}
    function calldata_public(bytes calldata) public {}

    function memory_callback(function(bytes memory) internal) private pure {}
    function memory_callback(function(bytes memory) external) private pure {}

    function calldata_callback(function(bytes calldata) internal) private pure {}
    // This is not referenced because calling an external function with calldata parameters makes no sense
    function calldata_callback(function(bytes calldata) external) private pure {}

    function test() internal view {
        memory_callback(memory_internal);
        memory_callback(this.memory_external);  // resolves to the external variant
        memory_callback(memory_private);
        memory_callback(memory_public);
        memory_callback(this.memory_public);    // resolves to the external variant

        memory_callback(this.calldata_external);
        memory_callback(this.calldata_public);
        // Calls with other variants of calldata_* functions are not valid

        // calldata_callback cannot be called with any of the memory_* functions

        // All of these resolve to the internal parameter variant
        calldata_callback(calldata_internal);
        calldata_callback(calldata_private);
        calldata_callback(calldata_public);
    }
}
