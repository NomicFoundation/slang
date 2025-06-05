contract Test {
    function test(string storage a) internal {
        bytes storage x = bytes(a);
        bytes1 b;
        x.push(b);
        x.pop();
    }
}
