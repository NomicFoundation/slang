contract Test {
    struct Resource {
        uint value;
        //   ^def:1
    }

    function test_call_output() public returns (uint) {
        return get_resource().value;
        //                    ^ref:1
        //     ^ref:2
    }

    function get_resource() internal returns (Resource memory) {
        //   ^def:2
        return Resource(1);
    }
}
