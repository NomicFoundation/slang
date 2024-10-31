contract Test {
    function test(bytes memory data) public {
        0x2d3fC875de7Fe7Da43AD0afa0E7023c9B91D06b1.delegatecall(data);
    }
}
