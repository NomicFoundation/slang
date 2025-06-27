contract Test {
    using Utils for bytes32;
    function test(bytes memory x) public pure {
        keccak256(x).post();
    }
}

library Utils {
    function post(bytes32 x) public returns (bytes32) {
        return x;
    }
}
