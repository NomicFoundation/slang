contract Base {
    function in_base() internal {}
}
contract Middle is Base {}
contract Test is Middle {
    function test() public {
        Base.in_base();
    }
}
