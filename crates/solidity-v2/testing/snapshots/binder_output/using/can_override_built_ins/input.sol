library Lib {
    function push(bytes memory, uint x) internal returns (bytes memory) {}
}

contract Test {
    using Lib for bytes;

    bytes b;

    function test() internal {
        b.push(1).push(2);
        b.length;
    }
}
