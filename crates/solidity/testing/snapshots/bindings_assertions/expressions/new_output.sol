contract Test {
    function test_new_output() public returns (uint) {
        return new Container().cell().value;
        //                            ^ref:2
        //                     ^ref:3
        //         ^ref:1
    }
}

contract Container {
    //   ^def:1
    struct Resource {
        uint value;
        //   ^def:2
    }

    function cell() public returns (Resource memory) { return Resource(1); }
    //       ^def:3
}
