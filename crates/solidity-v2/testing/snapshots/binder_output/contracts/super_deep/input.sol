contract Base {
    function in_base() public {}
}
contract Middle is Base {}
contract Test is Middle {
    function in_base() public {
        super.in_base();
    }
}
