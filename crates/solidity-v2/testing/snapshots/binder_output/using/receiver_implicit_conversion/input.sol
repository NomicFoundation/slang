library Lib {
    function nopCalldata(bytes calldata) external {}
    function nopMemory(bytes memory) external {}
}

contract Test {
    using Lib for bytes;
    bytes s;
    function test(bytes calldata c) public {
        bytes memory m;
        m.nopCalldata();
        m.nopMemory();
        c.nopCalldata();
        c.nopMemory();
        s.nopCalldata();
        s.nopMemory();
    }
}
