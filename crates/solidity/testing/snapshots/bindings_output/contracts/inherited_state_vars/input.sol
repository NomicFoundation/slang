contract Base {
    int in_base;
}
contract Middle is Base {
    int in_middle;
}
contract Test is Middle {
    function test() public {
        in_base;
        in_middle;
    }
}
