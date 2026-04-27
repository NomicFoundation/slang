contract Test {
    function test_non_decl_init() public {
        int z;
        int w = 0;
        for (z = 10; z > 0; w += z) {
            z--;
        }
    }
}
