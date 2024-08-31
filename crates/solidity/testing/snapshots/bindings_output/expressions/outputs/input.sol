contract Test {
    function test_new() public returns (uint) {
        return new Container().cell().value;
    }
    function test_call() public returns (uint) {
        return Utils.create().value;
    }
}

contract Container {
    function cell() public returns (Utils.Resource memory) {
        return Utils.create();
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
