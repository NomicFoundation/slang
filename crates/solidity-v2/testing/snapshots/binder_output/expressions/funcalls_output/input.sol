contract Test {
    function test_call() public returns (uint) {
        return Utils.create().value;
    }
}

library Utils {
    struct Resource {
        uint value;
    }

    function create() public returns (Resource memory) {
        return Resource(1);
    }
}
