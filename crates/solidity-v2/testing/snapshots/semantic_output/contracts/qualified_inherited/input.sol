contract Base {
    bool renounced;
}
contract Test is Base {
    function test() public {
        Base.renounced = true;
    }
}
