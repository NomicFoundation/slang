contract Base {
    function in_base() {}
}
contract Middle is Base {}
contract Test is Middle {
    function in_base() {
        super.in_base();
    }
}
