contract Test {
    function test_new() public returns (uint) {
        return new Container().cell().value;
    }
}

contract Container {
    struct Resource {
        uint value;
    }

    function cell() public returns (Resource memory) {
        return Resource(1);
    }
}
